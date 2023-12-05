use anyhow::{bail, ensure, Result};
use bitvec::prelude::*;
use mozjpeg_sys::{
  boolean, jpeg_decompress_struct, jpeg_destroy_decompress, jpeg_finish_decompress, jpeg_read_coefficients,
  jpeg_read_header,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::options::ExtraArgs;
use crate::utils;

use super::Decoder;

pub struct JpegDecoder {
  cinfo: jpeg_decompress_struct,
  total_size: usize,
  blocks: utils::jpeg::Blocks,
  rng: ChaCha20Rng,
  extra: ExtraArgs,
}

impl JpegDecoder {
  pub fn new(image_buffer: &Vec<u8>, extra: ExtraArgs) -> Result<Self> {
    ensure!(
      extra.depth + extra.bits <= 8,
      "invalid depth and bits: {} + {} > 8",
      extra.depth,
      extra.bits
    );
    let (cinfo, total_size, blocks) = unsafe {
      let mut cinfo = utils::jpeg::decompress(image_buffer)?;
      jpeg_read_header(&mut cinfo, true as boolean);
      let coefs_ptr = jpeg_read_coefficients(&mut cinfo);
      let (blocks, total_size) = utils::jpeg::get_blocks(&mut cinfo, coefs_ptr, extra.jpeg_comp)?;

      let total_size = if extra.selective {
        utils::jpeg::selective_total_size(&extra, &blocks)
      } else {
        total_size
      };

      (cinfo, total_size, blocks)
    };

    Ok(Self {
      cinfo,
      total_size,
      blocks,
      rng: ChaCha20Rng::from_seed(Seeder::from(extra.key.clone()).make_seed()),
      extra,
    })
  }
}

impl Decoder for JpegDecoder {
  fn total_size(&self) -> usize {
    (self.total_size - 32) * self.extra().bits
  }

  fn extra(&self) -> &ExtraArgs {
    &self.extra
  }

  fn read(&mut self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let rng = &mut self.rng;

    let mut image_iter = self
      .blocks
      .iter()
      .enumerate()
      .filter(|(idx, coef)| !utils::jpeg::selective_check(&self.extra, *idx, **coef))
      .map(|(_, coef)| coef);

    let mut data_iter = data.iter_mut();
    let mask = !(u16::max_value() << self.extra.bits) as i16;

    if seek > 0 {
      image_iter.nth(seek - 1);
    }

    while let Some(coef) = image_iter.nth(utils::iter::rand_step(rng, max_step)) {
      let value = *coef >> self.extra.depth & mask;
      let mut value = value.reverse_bits() >> (16 - self.extra.bits);
      for _ in 0..self.extra.bits {
        let mut bit = match data_iter.next() {
          Some(bit) => bit,
          None => return Ok(()),
        };
        *bit = (value & 1) == 1;
        value >>= 1;
      }
    }

    if data_iter.next().is_some() {
      bail!("image ended but data not");
    }
    Ok(())
  }
}

impl Drop for JpegDecoder {
  fn drop(&mut self) {
    unsafe {
      jpeg_finish_decompress(&mut self.cinfo);
      jpeg_destroy_decompress(&mut self.cinfo);
    }
  }
}
