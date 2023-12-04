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
  fn total_size(&self) -> usize {
    self.image.width() as usize * self.image.height() as usize * self.image.color().channel_count() as usize
  }

  fn extra(&self) -> &ExtraArgs {
    &self.extra
  }

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

    if data_iter.next().is_some() {
      bail!("image ended but data not");
    }
    Ok(())
  }
}
