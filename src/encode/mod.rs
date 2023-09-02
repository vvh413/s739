pub mod jpeg;
pub mod png;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use crate::cli::{Data, EncodeArgs, PngOpts};
use anyhow::{bail, Result};
use bitvec::slice::BitSlice;

use self::jpeg::JpegEncoder;
use self::png::PngEncoder;

pub trait Encoder {
  fn write(&mut self, data: &BitSlice<u8>, seek: usize, max_step: usize) -> Result<()>;
  fn write_data(&mut self, data: &[u8]) -> Result<()>;
  fn save(&mut self, output: PathBuf, png_opts: PngOpts) -> Result<()>;
}

pub fn new_encoder(input: PathBuf, key: Option<String>) -> Result<Box<dyn Encoder>> {
  match image::ImageFormat::from_path(input.clone())? {
    image::ImageFormat::Png => Ok(Box::new(PngEncoder::new(image::open(input)?, key)?)),
    image::ImageFormat::Jpeg => Ok(Box::new(JpegEncoder::new(input, key)?)),
    _ => bail!("invalid image format"),
  }
}

fn read_data(data: Data) -> Result<Vec<u8>> {
  let mut buf = Vec::new();
  match (data.text, data.file, data.stdin) {
    (Some(text), _, _) => {
      buf = text.as_bytes().to_vec();
    }
    (_, Some(file), _) => {
      let _ = BufReader::new(File::open(file)?).read_to_end(&mut buf)?;
    }
    (_, _, true) => {
      let _ = std::io::stdin().read_to_end(&mut buf)?;
    }
    _ => unreachable!(),
  };
  Ok(buf)
}

pub fn encode(args: EncodeArgs) -> Result<()> {
  let EncodeArgs {
    input,
    output,
    data,
    png_opts,
    key,
  } = args;

  let mut encoder: Box<dyn Encoder> = new_encoder(input, key)?;
  let data = read_data(data)?;
  encoder.write_data(&data)?;
  encoder.save(output, png_opts)?;

  Ok(())
}
