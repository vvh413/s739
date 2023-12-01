pub mod jpeg;
pub mod png;

use std::path::PathBuf;

use anyhow::{bail, Result};
use bitvec::slice::BitSlice;

use crate::options::ExtraArgs;

use self::jpeg::JpegDecoder;
use self::png::PngDecoder;

pub trait Decoder {
  fn read(&mut self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()>;
  fn read_data(&mut self) -> Result<Vec<u8>>;
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
