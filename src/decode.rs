use std::fs::File;
use std::io::Write;
use std::slice::Iter;

use anyhow::{bail, ensure, Result};
use bitvec::prelude::*;
use image::DynamicImage;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::cli::DecodeArgs;

pub struct Decoder<'a> {
  image_iter: Iter<'a, u8>,
  rng: ChaCha20Rng,
}

impl<'a> Decoder<'a> {
  pub fn new(image: &'a DynamicImage, key: Option<String>) -> Result<Self> {
    let image_iter = match image {
      DynamicImage::ImageRgb8(img_buf) => img_buf.iter(),
      DynamicImage::ImageRgba8(img_buf) => img_buf.iter(),
      _ => panic!("invalid color format"),
    };

    Ok(Self {
      image_iter,
      rng: ChaCha20Rng::from_seed(Seeder::from(key).make_seed()),
    })
  }
  pub fn read(&mut self, buf: &mut BitSlice<u8>, max_step: usize) -> Result<()> {
    for mut bit in buf.iter_mut() {
      let step = if max_step > 1 {
        self.rng.gen_range(0..max_step)
      } else {
        0
      };
      let pixel = match self.image_iter.nth(step) {
        Some(pixel) => pixel,
        None => bail!("read: image ended, but data not"),
      };
      bit.set((pixel & 0x1) == 1);
    }
    Ok(())
  }

  pub fn read_data(&mut self) -> Result<Vec<u8>> {
    let size = bits![mut u8, Lsb0; 0u8; 32];
    self.read(size, 0)?;
    let size: usize = size.load();

    let max_step = self.image_iter.len() / (size << 3);
    ensure!(max_step > 0, "invalid data size");

    let mut data = vec![0u8; size];
    self.read(data.view_bits_mut(), max_step)?;

    Ok(data)
  }
}

pub fn decode(args: DecodeArgs) -> Result<()> {
  let DecodeArgs { input, file, key } = args;
  let input_image = image::open(input)?;

  let mut decoder = Decoder::new(&input_image, key)?;
  let data = decoder.read_data()?;

  match file {
    Some(file) => File::create(file)?.write_all(&data),
    None => std::io::stdout().write_all(&data),
  }?;

  Ok(())
}
