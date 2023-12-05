use anyhow::{bail, ensure, Result};
use bitvec::slice::BitSlice;
use mozjpeg_sys::{
  boolean, jpeg_copy_critical_parameters, jpeg_decompress_struct, jpeg_destroy_compress, jpeg_destroy_decompress,
  jpeg_finish_compress, jpeg_finish_decompress, jpeg_read_coefficients, jpeg_read_header, jpeg_write_coefficients,
  jvirt_barray_control,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::options::{ExtraArgs, ImageOptions};
use crate::utils;

use super::Encoder;

pub struct JpegEncoder {
  cinfo: jpeg_decompress_struct,
  coefs_ptr: *mut *mut jvirt_barray_control,
  total_size: usize,
  blocks: Vec<(*mut [i16; 64], u32)>,
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
    let mut seek = seek;
    let mut step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
    let mut data_iter = data.iter();
    let mask = (0xffffu16 << self.extra.bits).rotate_left(self.extra.depth as u32) as i16;

    for (block, width) in self.blocks.iter() {
      for blk_x in 0..*width {
        for (idx, coef) in unsafe { (*block.offset(blk_x as isize)).iter_mut() }.enumerate() {
          if seek > 0 {
            seek -= 1;
            continue;
          }
          if step > 0 {
            step -= 1;
            continue;
          }
          if utils::jpeg::selective_check(&self.extra, idx, *coef as usize) {
            continue;
          }

          let mut bits = 0;
          for i in (0..self.extra.bits).rev() {
            let bit = match data_iter.next() {
              Some(bit) => bit,
              None => {
                if i == self.extra.bits {
                  return Ok(());
                } else {
                  break;
                }
              }
            };
            bits |= (if *bit { 1 } else { 0 }) << i;
          }
          *coef = (*coef & mask) | (bits << self.extra.depth);

          step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
        }
      }
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
