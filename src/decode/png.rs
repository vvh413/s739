use anyhow::{bail, ensure, Result};
use bitvec::prelude::*;
use image::DynamicImage;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::options::ExtraArgs;

use super::Decoder;

pub struct PngDecoder {
  image: DynamicImage,
  rng: ChaCha20Rng,
  extra: ExtraArgs,
}

impl PngDecoder {
  pub fn new(image: DynamicImage, extra: ExtraArgs) -> Result<Self> {
    ensure!(
      extra.depth + extra.lsbs <= 8,
      "invalid depth and LSBs: {} + {} > 8",
      extra.depth,
      extra.lsbs
    );
    Ok(Self {
      image,
      rng: ChaCha20Rng::from_seed(Seeder::from(extra.key.clone()).make_seed()),
      extra,
    })
  }
}

impl Decoder for PngDecoder {
  fn read(&mut self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let mut image_iter = match &self.image {
      DynamicImage::ImageRgb8(img_buf) => img_buf.iter(),
      DynamicImage::ImageRgba8(img_buf) => img_buf.iter(),
      _ => bail!("invalid color format"),
    };
    let rng = &mut self.rng;
    let mut step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
    let mut data_iter = data.iter_mut();

    if seek > 0 {
      image_iter.nth(seek - 1);
    }

    while let Some(pixel) = image_iter.nth(step) {
      let value = *pixel >> self.extra.depth & !(0xff << self.extra.lsbs);
      let mut value = value.reverse_bits() >> (8 - self.extra.lsbs);
      for _ in 0..self.extra.lsbs {
        let mut bit = match data_iter.next() {
          Some(bit) => bit,
          None => return Ok(()),
        };
        *bit = (value & 1) == 1;
        value >>= 1;
      }
      step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
    }
    Ok(())
  }

  fn read_data(&mut self) -> Result<Vec<u8>> {
    let size = bits![mut u8, Lsb0; 0u8; 32];
    self.read(size, 0, 0)?;
    let size: usize = size.load();
    ensure!(size != 0, "no data found");

    let max_step = (self.image.width() * self.image.height() * self.image.color().channel_count() as u32 - 32) as usize
      / (size << 3);
    ensure!(max_step > 0, "invalid data size");

    let mut data = vec![0u8; size];
    self.read(data.view_bits_mut(), 32, max_step)?;

    Ok(data)
  }
}
