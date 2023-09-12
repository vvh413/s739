pub mod jpeg;
pub mod png;

use std::path::PathBuf;

use crate::options::ImageOptions;
use anyhow::{bail, Result};
use bitvec::slice::BitSlice;

use self::jpeg::JpegEncoder;
use self::png::PngEncoder;

pub trait Encoder {
  fn write(&mut self, data: &BitSlice<u8>, seek: usize, max_step: usize) -> Result<()>;
  fn write_data(&mut self, data: &[u8]) -> Result<()>;
  fn encode_image(&mut self, image_opts: ImageOptions) -> Result<Vec<u8>>;
}

pub fn new_encoder(input: PathBuf, key: Option<String>) -> Result<Box<dyn Encoder>> {
  match image::ImageFormat::from_path(input.clone())? {
    image::ImageFormat::Png => Ok(Box::new(PngEncoder::new(image::open(input)?, key)?)),
    image::ImageFormat::Jpeg => Ok(Box::new(JpegEncoder::new(&std::fs::read(input)?, key)?)),
    _ => bail!("invalid image format"),
  }
}
