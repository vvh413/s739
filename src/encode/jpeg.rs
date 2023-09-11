use anyhow::{ensure, Result};
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use mozjpeg_sys::{
  boolean, jpeg_copy_critical_parameters, jpeg_decompress_struct, jpeg_destroy_compress, jpeg_destroy_decompress,
  jpeg_finish_compress, jpeg_finish_decompress, jpeg_read_coefficients, jpeg_read_header, jpeg_write_coefficients,
  jvirt_barray_control,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::cli::ImageOpts;
use crate::utils::{compress, decompress, get_total_size};

use super::Encoder;

pub struct JpegEncoder {
  cinfo: jpeg_decompress_struct,
  coefs_ptr: *mut *mut jvirt_barray_control,
  rng: ChaCha20Rng,
}

impl JpegEncoder {
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

impl Encoder for JpegEncoder {
  fn write(&mut self, data: &BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let mut data_iter = data.iter();
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
            1,
          );
          for offset_y in 0..(*comp_info).v_samp_factor {
            let block = *buffer.offset(offset_y as isize);
            for blk_x in 0..(*comp_info).width_in_blocks {
              for coef in (*block.offset(blk_x as isize)).iter_mut() {
                if seek > 0 {
                  seek -= 1;
                  continue;
                }
                if step > 0 {
                  step -= 1;
                  continue;
                }

                let bit = match data_iter.next() {
                  Some(bit) => bit,
                  None => return Ok(()),
                };
                *coef = (*coef & -2) | (if *bit { 1 } else { 0 });

                step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
              }
            }
          }
        }
      }
    }
    Ok(())
  }
  fn write_data(&mut self, data: &[u8]) -> Result<()> {
    let max_step = unsafe { (get_total_size(&self.cinfo) - 32) / (data.len() << 3) };
    ensure!(max_step > 0, "invalid data size");

    self.write((data.len() as u32).to_le_bytes().view_bits(), 0, 0)?;
    self.write(data.view_bits(), 32, max_step)?;

    Ok(())
  }

  fn encode_image(&mut self, _image_opts: ImageOpts) -> Result<Vec<u8>> {
    let buffer: Vec<u8> = unsafe {
      let buffer_ptr: *mut *mut u8 = &mut [0u8; 0].as_mut_ptr();
      let buffer_size: *mut u64 = &mut 0;
      let mut dstinfo = compress(buffer_ptr, buffer_size)?;

      jpeg_copy_critical_parameters(&self.cinfo, &mut dstinfo);

      jpeg_write_coefficients(&mut dstinfo, self.coefs_ptr);

      jpeg_finish_compress(&mut dstinfo);
      jpeg_destroy_compress(&mut dstinfo);
      jpeg_finish_decompress(&mut self.cinfo);
      jpeg_destroy_decompress(&mut self.cinfo);

      Vec::from_raw_parts(*buffer_ptr, *buffer_size as usize, *buffer_size as usize)
    };
    Ok(buffer)
  }
}
