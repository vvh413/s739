pub mod jpeg;
pub mod png;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{bail, Result};
use bitvec::slice::BitSlice;

use crate::cli::DecodeArgs;

use self::jpeg::JpegDecoder;
use self::png::PngDecoder;

pub trait Decoder {
  fn read(&mut self, buf: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()>;
  fn read_data(&mut self) -> Result<Vec<u8>>;
}

pub fn new_decoder(input: PathBuf, key: Option<String>) -> Result<Box<dyn Decoder>> {
  match image::ImageFormat::from_path(input.clone())? {
    image::ImageFormat::Png => Ok(Box::new(PngDecoder::new(image::open(input)?, key)?)),
    image::ImageFormat::Jpeg => Ok(Box::new(JpegDecoder::new(input, key)?)),
    _ => bail!("invalid image format"),
  }
}

pub fn decode(args: DecodeArgs) -> Result<()> {
  let DecodeArgs { input, file, key } = args;

  let mut decoder = new_decoder(input, key)?;
  let data = decoder.read_data()?;

  match file {
    Some(file) => File::create(file)?.write_all(&data),
    None => std::io::stdout().write_all(&data),
  }?;

  Ok(())
}
