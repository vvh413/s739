use anyhow::{bail, ensure, Result};
use bitvec::prelude::*;
use mozjpeg_sys::{jpeg_decompress_struct, jpeg_destroy_decompress, jpeg_finish_decompress};
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

    let (cinfo, _, total_size, blocks) = unsafe { utils::jpeg::decompress(image_buffer, &extra)? };

    let total_size = if extra.selective {
      blocks.iter(extra.clone()).count()
    } else {
      total_size
    };

    Ok(Self {
      cinfo,
      total_size,
      blocks,
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

  fn read(&self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let mut rng = ChaCha20Rng::from_seed(Seeder::from(self.extra.key.clone()).make_seed());
    let mut image_iter = self.blocks.iter(self.extra().clone());
    let mut data_iter = data.iter_mut();
    let mask = !(u16::max_value() << self.extra.bits) as i16;

    if seek > 0 {
      image_iter.nth(seek - 1);
    }

    while let Some(coef) = image_iter.nth(utils::iter::rand_step(&mut rng, max_step)) {
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
