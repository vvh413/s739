use anyhow::{bail, ensure, Result};
use bitvec::prelude::*;
use mozjpeg_sys::{
  boolean, jpeg_decompress_struct, jpeg_destroy_decompress, jpeg_finish_decompress, jpeg_read_coefficients,
  jpeg_read_header,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::options::ExtraArgs;
use crate::utils;

use super::Decoder;

pub struct JpegDecoder {
  cinfo: jpeg_decompress_struct,
  total_size: usize,
  blocks: Vec<(*mut [i16; 64], u32)>,
  rng: ChaCha20Rng,
  extra: ExtraArgs,
}

impl JpegDecoder {
  pub fn new(image_buffer: &Vec<u8>, extra: ExtraArgs) -> Result<Self> {
    ensure!(
      extra.depth + extra.lsbs <= 8,
      "invalid depth and LSBs: {} + {} > 8",
      extra.depth,
      extra.lsbs
    );
    let (cinfo, total_size, blocks) = unsafe {
      let mut cinfo = utils::jpeg::decompress(image_buffer)?;
      jpeg_read_header(&mut cinfo, true as boolean);
      let coefs_ptr = jpeg_read_coefficients(&mut cinfo);
      let (blocks, total_size) = utils::jpeg::get_blocks(&mut cinfo, coefs_ptr, extra.jpeg_comp)?;

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
    self.total_size
  }

  fn extra(&self) -> &ExtraArgs {
    &self.extra
  }

  fn read(&mut self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let rng = &mut self.rng;
    let mut seek = seek;
    let mut step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
    let mut data_iter = data.iter_mut();

    for (block, width) in self.blocks.iter() {
      for blk_x in 0..*width {
        for (idx, coef) in unsafe { *block.offset(blk_x as isize) }.iter().enumerate() {
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

          let value = *coef as u16 >> self.extra.depth & !(0xffff << self.extra.lsbs);
          let mut value = value.reverse_bits() >> (16 - self.extra.lsbs);
          for _ in 0..self.extra.lsbs {
            let mut bit = match data_iter.next() {
              Some(bit) => bit,
              None => return Ok(()),
            };
            *bit = (value & 1) == 1;
            value >>= 1;
          }

          step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
        }
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
