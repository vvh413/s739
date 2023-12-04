pub mod jpeg;
pub mod png;

use std::path::PathBuf;

use crate::options::{ExtraArgs, ImageOptions};
use anyhow::{bail, ensure, Result};
use bitvec::slice::BitSlice;
use bitvec::view::BitView;

use self::jpeg::JpegEncoder;
use self::png::PngEncoder;

pub trait Encoder {
  fn write(&mut self, data: &BitSlice<u8>, seek: usize, max_step: usize) -> Result<()>;
  fn encode_image(&mut self, image_opts: ImageOptions) -> Result<Vec<u8>>;
  fn total_size(&self) -> usize;
  fn extra(&self) -> &ExtraArgs;

  fn write_data(&mut self, data: &[u8]) -> Result<()> {
    self.check_size(data.len())?;

    self.write((data.len() as u32).to_le_bytes().view_bits(), 0, 0)?;
    self.write(data.view_bits(), 32, self.max_step(data.len()))?;

    Ok(())
  }

  fn check_size(&self, data_size: usize) -> Result<()> {
    ensure!((data_size << 3) != 0, "data is empty");
    let total_size = self.total_size() - 32;
    let data_size = (data_size << 3) / self.extra().lsbs + 1;
    if data_size > total_size {
      bail!("too much data: data {data_size} vs image {total_size}")
    }
    Ok(())
  }

  fn max_step(&self, data_size: usize) -> usize {
    match self.extra().max_step {
      Some(max_step) => max_step,
      None => (self.total_size() - 32) / ((data_size << 3) / self.extra().lsbs + 1),
    }
  }
}

pub fn new_encoder(input: PathBuf, extra_args: ExtraArgs) -> Result<Box<dyn Encoder>> {
  let image_buf = std::fs::read(input)?;
  match image::guess_format(&image_buf)? {
    image::ImageFormat::Png => Ok(Box::new(PngEncoder::new(
      image::load_from_memory(&image_buf)?,
      extra_args,
    )?)),
    image::ImageFormat::Jpeg => Ok(Box::new(JpegEncoder::new(&image_buf, extra_args)?)),
    _ => bail!("invalid image format"),
  }
}
