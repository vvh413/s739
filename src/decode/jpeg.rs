use anyhow::{ensure, Result};
use bitvec::prelude::*;
use mozjpeg_sys::{
  boolean, jpeg_decompress_struct, jpeg_destroy_decompress, jpeg_finish_decompress, jpeg_read_coefficients,
  jpeg_read_header, jvirt_barray_control,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::utils::{decompress, get_total_size};

use super::Decoder;

pub struct JpegDecoder {
  cinfo: jpeg_decompress_struct,
  coefs_ptr: *mut *mut jvirt_barray_control,
  rng: ChaCha20Rng,
}

impl JpegDecoder {
  pub fn new(image_buffer: &Vec<u8>, key: Option<String>) -> Result<Self> {
    let (cinfo, coefs_ptr) = unsafe {
      let mut cinfo = decompress(image_buffer)?;
      jpeg_read_header(&mut cinfo, true as boolean);
      let coefs_ptr = jpeg_read_coefficients(&mut cinfo);
      (cinfo, coefs_ptr)
    };

    Ok(Self {
      cinfo,
      coefs_ptr,
      rng: ChaCha20Rng::from_seed(Seeder::from(key).make_seed()),
    })
  }
}

impl Decoder for JpegDecoder {
  fn read(&mut self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let mut data_iter = data.iter_mut();
    let mut seek = seek;
    let rng = &mut self.rng;
    let mut step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };

    unsafe {
      for comp in 0..self.cinfo.num_components as isize {
        let comp_info = self.cinfo.comp_info.offset(comp);
        for blk_y in (0..(*comp_info).height_in_blocks).step_by((*self.cinfo.comp_info).v_samp_factor as usize) {
          let buffer = (*self.cinfo.common.mem).access_virt_barray.unwrap()(
            &mut self.cinfo.common,
            *self.coefs_ptr.offset(comp),
            blk_y,
            (*comp_info).v_samp_factor as u32,
            0,
          );
          for offset_y in 0..(*comp_info).v_samp_factor {
            let block = *buffer.offset(offset_y as isize);
            for blk_x in 0..(*comp_info).width_in_blocks {
              for coef in (*block.offset(blk_x as isize)).iter() {
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
        }
      }
    }
    Ok(())
  }

  fn read_data(&mut self) -> Result<Vec<u8>> {
    let size = bits![mut u8, Lsb0; 0u8; 32];
    self.read(size, 0, 0)?;
    let size: usize = size.load();

    let max_step = unsafe { (get_total_size(&self.cinfo) - 32) / (size << 3) };
    ensure!(max_step > 0, "invalid data size");

    let mut data = vec![0u8; size];
    self.read(data.view_bits_mut(), 32, max_step)?;

    unsafe {
      jpeg_finish_decompress(&mut self.cinfo);
      jpeg_destroy_decompress(&mut self.cinfo);
    }

    Ok(data)
  }
}
