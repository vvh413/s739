use anyhow::{ensure, Result};
use bitvec::prelude::*;
use mozjpeg_sys::{
  boolean, jpeg_decompress_struct, jpeg_destroy_decompress, jpeg_finish_decompress, jpeg_read_coefficients,
  jpeg_read_header,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::utils::{decompress, get_blocks, get_total_size};

use super::Decoder;

pub struct JpegDecoder {
  cinfo: jpeg_decompress_struct,
  total_size: usize,
  blocks: Vec<(*mut [i16; 64], u32)>,
  rng: ChaCha20Rng,
}

impl JpegDecoder {
  pub fn new(image_buffer: &Vec<u8>, key: Option<String>) -> Result<Self> {
    let (cinfo, total_size, blocks) = unsafe {
      let mut cinfo = decompress(image_buffer)?;
      jpeg_read_header(&mut cinfo, true as boolean);
      let coefs_ptr = jpeg_read_coefficients(&mut cinfo);
      let total_size = get_total_size(&cinfo);
      let blocks = get_blocks(&mut cinfo, coefs_ptr);

      (cinfo, total_size, blocks)
    };

    Ok(Self {
      cinfo,
      total_size,
      blocks,
      rng: ChaCha20Rng::from_seed(Seeder::from(key).make_seed()),
    })
  }
}

impl Decoder for JpegDecoder {
  fn read(&mut self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let rng = &mut self.rng;
    let mut seek = seek;
    let mut step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
    let mut data_iter = data.iter_mut();

    for (block, width) in self.blocks.iter() {
      for blk_x in 0..*width {
        for coef in unsafe { *block.offset(blk_x as isize) }.iter() {
          if seek > 0 {
            seek -= 1;
            continue;
          }
          if step > 0 {
            step -= 1;
            continue;
          }

          let mut bit = match data_iter.next() {
            Some(bit) => bit,
            None => return Ok(()),
          };
          *bit = *coef & 1 == 1;

          step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
        }
      }
    }
    Ok(())
  }

  fn read_data(&mut self) -> Result<Vec<u8>> {
    let size = bits![mut u8, Lsb0; 0u8; 32];
    self.read(size, 0, 0)?;
    let size: usize = size.load();
    ensure!(size != 0, "no data found");

    let max_step = (self.total_size - 32) / (size << 3);
    ensure!(max_step > 0, "invalid data size");

    let mut data = vec![0u8; size];
    self.read(data.view_bits_mut(), 32, max_step)?;

    Ok(data)
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