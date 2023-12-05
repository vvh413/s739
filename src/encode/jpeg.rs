use anyhow::{bail, ensure, Result};
use bitvec::slice::BitSlice;
use mozjpeg_sys::{
  boolean, jpeg_copy_critical_parameters, jpeg_decompress_struct, jpeg_destroy_compress, jpeg_destroy_decompress,
  jpeg_finish_compress, jpeg_finish_decompress, jpeg_read_coefficients, jpeg_read_header, jpeg_write_coefficients,
  jvirt_barray_control,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::options::{ExtraArgs, ImageOptions};
use crate::utils;

use super::Encoder;

pub struct JpegEncoder {
  cinfo: jpeg_decompress_struct,
  coefs_ptr: *mut *mut jvirt_barray_control,
  total_size: usize,
  blocks: utils::jpeg::Blocks,
  rng: ChaCha20Rng,
  extra: ExtraArgs,
}

impl JpegEncoder {
  pub fn new(image_buffer: &Vec<u8>, extra: ExtraArgs) -> Result<Self> {
    ensure!(
      extra.depth + extra.bits <= 8,
      "invalid depth and bits: {} + {} > 8",
      extra.depth,
      extra.bits
    );
    let (cinfo, coefs_ptr, total_size, blocks) = unsafe {
      let mut cinfo = utils::jpeg::decompress(image_buffer)?;
      jpeg_read_header(&mut cinfo, true as boolean);

      let coefs_ptr = jpeg_read_coefficients(&mut cinfo);
      let (blocks, total_size) = utils::jpeg::get_blocks(&mut cinfo, coefs_ptr, extra.jpeg_comp)?;

      let total_size = if extra.selective {
        utils::jpeg::selective_total_size(&extra, &blocks)
      } else {
        total_size
      };

      (cinfo, coefs_ptr, total_size, blocks)
    };

    Ok(Self {
      cinfo,
      coefs_ptr,
      total_size,
      blocks,
      rng: ChaCha20Rng::from_seed(Seeder::from(extra.key.clone()).make_seed()),
      extra,
    })
  }
}

impl Encoder for JpegEncoder {
  fn total_size(&self) -> usize {
    (self.total_size - 32) * self.extra().bits
  }

  fn extra(&self) -> &ExtraArgs {
    &self.extra
  }

  fn write(&mut self, data: &BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let rng = &mut self.rng;

    let mut image_iter = self
      .blocks
      .iter_mut()
      .enumerate()
      .filter(|(idx, coef)| !utils::jpeg::selective_check(&self.extra, *idx, **coef))
      .map(|(_, coef)| coef);

    let mut data_iter = data.iter();
    let mask = (u16::max_value() << self.extra.bits).rotate_left(self.extra.depth as u32) as i16;

    if seek > 0 {
      image_iter.nth(seek - 1);
    }

    while let Some(coef) = image_iter.nth(utils::iter::rand_step(rng, max_step)) {
      let bits = match utils::iter::get_n_data_bits(&mut data_iter, self.extra.bits) {
        Some(bits) => bits as i16,
        None => return Ok(()),
      };
      *coef = (*coef & mask) | (bits << self.extra.depth);
    }

    if data_iter.next().is_some() {
      bail!("image ended but data not");
    }
    Ok(())
  }

  fn encode_image(&mut self, image_opts: ImageOptions) -> Result<Vec<u8>> {
    let buffer: Vec<u8> = unsafe {
      let buffer_ptr: *mut *mut u8 = &mut [0u8; 0].as_mut_ptr();
      let buffer_size: *mut libc::c_ulong = &mut 0;
      let mut dstinfo = utils::jpeg::compress(buffer_ptr, buffer_size);

      utils::jpeg::set_options(&mut dstinfo, image_opts.jpeg);
      jpeg_copy_critical_parameters(&self.cinfo, &mut dstinfo);

      jpeg_write_coefficients(&mut dstinfo, self.coefs_ptr);

      jpeg_finish_compress(&mut dstinfo);
      jpeg_destroy_compress(&mut dstinfo);

      Vec::from_raw_parts(*buffer_ptr, *buffer_size as usize, *buffer_size as usize)
    };
    Ok(buffer)
  }
}

impl Drop for JpegEncoder {
  fn drop(&mut self) {
    unsafe {
      jpeg_finish_decompress(&mut self.cinfo);
      jpeg_destroy_decompress(&mut self.cinfo);
    }
  }
}
