pub mod jpeg;
pub mod png;

use std::path::PathBuf;

use anyhow::{bail, Result};
use bitvec::slice::BitSlice;

use self::jpeg::JpegDecoder;
use self::png::PngDecoder;

pub trait Decoder {
  fn read(&mut self, buf: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()>;
  fn read_data(&mut self) -> Result<Vec<u8>>;
}

pub fn new_decoder(input: PathBuf, key: Option<String>, jpeg_comp: Option<u8>) -> Result<Box<dyn Decoder>> {
  let image_buf = std::fs::read(input)?;
  match image::guess_format(&image_buf)? {
    image::ImageFormat::Png => Ok(Box::new(PngDecoder::new(image::load_from_memory(&image_buf)?, key)?)),
    image::ImageFormat::Jpeg => Ok(Box::new(JpegDecoder::new(&image_buf, key, jpeg_comp)?)),
    _ => bail!("invalid image format"),
  }
}
