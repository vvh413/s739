pub mod jpeg;
pub mod png;

use std::path::PathBuf;

use anyhow::{bail, ensure, Result};
use bitvec::bits;
use bitvec::prelude::*;

use crate::options::ExtraArgs;

use self::jpeg::JpegDecoder;
use self::png::PngDecoder;

pub trait Decoder {
  fn read(&mut self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()>;
  fn total_size(&self) -> usize;
  fn extra(&self) -> &ExtraArgs;

  fn read_data(&mut self) -> Result<Vec<u8>> {
    let size = bits![mut u8, Lsb0; 0u8; 32];
    self.read(size, 0, 0)?;
    let size: usize = size.load();
    self.check_size(size)?;

    let (data_size, max_step) = self.data_size(size);
    let mut data = vec![0u8; data_size];
    self.read(data.view_bits_mut(), 32, max_step)?;

    Ok(data)
  }

  fn check_size(&self, data_size: usize) -> Result<()> {
    ensure!((data_size << 3) != 0, "no data found");
    let total_size = self.total_size() - 32;
    let data_size = (data_size << 3) / self.extra().lsbs + 1;
    if data_size > total_size {
      bail!("invalid data size: data {data_size} vs image {total_size}")
    }
    Ok(())
  }

  fn data_size(&self, data_size: usize) -> (usize, usize) {
    let total_size = self.total_size() - 32;
    match self.extra().max_step {
      Some(max_step) => ((total_size / max_step) >> 3, max_step),
      None => (data_size, total_size / ((data_size << 3) / self.extra().lsbs + 1)),
    }
  }
}

pub fn new_decoder(input: PathBuf, extra_args: ExtraArgs) -> Result<Box<dyn Decoder>> {
  let image_buf = std::fs::read(input)?;
  match image::guess_format(&image_buf)? {
    image::ImageFormat::Png => Ok(Box::new(PngDecoder::new(
      image::load_from_memory(&image_buf)?,
      extra_args,
    )?)),
    image::ImageFormat::Jpeg => Ok(Box::new(JpegDecoder::new(&image_buf, extra_args)?)),
    _ => bail!("invalid image format"),
  }
}
