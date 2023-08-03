use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::slice::IterMut;

use crate::cli::{Data, EncodeArgs};
use anyhow::{bail, ensure, Result};
use bitvec::prelude::*;
use image::{DynamicImage, ImageEncoder};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

pub struct Encoder<'a> {
  image_iter: IterMut<'a, u8>,
  data: &'a Vec<u8>,
  max_step: usize,
  rng: ChaCha20Rng,
}

impl<'a> Encoder<'a> {
  pub fn new(image: &'a mut DynamicImage, data: &'a Vec<u8>, key: Option<String>) -> Result<Self> {
    let image_iter = match image {
      DynamicImage::ImageRgb8(img_buf) => img_buf.iter_mut(),
      DynamicImage::ImageRgba8(img_buf) => img_buf.iter_mut(),
      _ => panic!("invalid color format"),
    };

    let max_step = (image_iter.len() - 32) / (data.len() << 3);
    ensure!(max_step > 0, "invalid data size");

    Ok(Self {
      image_iter,
      data,
      max_step,
      rng: ChaCha20Rng::from_seed(Seeder::from(key).make_seed()),
    })
  }

  pub fn write(&mut self, data: &BitSlice<u8, Lsb0>, max_step: usize) -> Result<()> {
    for bit in data {
      let step = if max_step > 1 {
        self.rng.gen_range(0..max_step)
      } else {
        0
      };
      match self.image_iter.nth(step) {
        Some(pixel) => pixel.view_bits_mut::<Lsb0>().set(0, *bit),
        None => bail!("write: image ended, but data not"),
      }
    }
    Ok(())
  }

  pub fn write_data(&mut self) -> Result<()> {
    self.write((self.data.len() as u32).to_le_bytes().view_bits(), 1)?;
    self.write(self.data.view_bits(), self.max_step)?;
    Ok(())
  }
}

fn read_data(data: Data) -> Result<Vec<u8>> {
  let mut buf = Vec::new();
  match (data.text, data.file, data.stdin) {
    (Some(text), _, _) => {
      buf = format!("{text}\n").as_bytes().to_vec();
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

  let mut input_image = image::open(input)?;
  let data = read_data(data)?;

  let mut encoder = Encoder::new(&mut input_image, &data, key)?;
  encoder.write_data()?;

  let buffered_file_write = &mut BufWriter::new(File::create(output)?);
  image::codecs::png::PngEncoder::new_with_quality(
    buffered_file_write,
    png_opts.png_compression.into(),
    png_opts.png_filter.into(),
  )
  .write_image(
    input_image.as_bytes(),
    input_image.width(),
    input_image.height(),
    input_image.color(),
  )?;

  Ok(())
}
