use crate::options::{ExtraArgs, ImageOptions};
use anyhow::{bail, ensure, Result};
use bitvec::prelude::*;
use image::{DynamicImage, ImageEncoder};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use super::Encoder;

pub struct PngEncoder {
  pub image: DynamicImage,
  rng: ChaCha20Rng,
  extra: ExtraArgs,
}

impl PngEncoder {
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

impl Encoder for PngEncoder {
  fn write(&mut self, data: &BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let mut image_iter = match &mut self.image {
      DynamicImage::ImageRgb8(img_buf) => img_buf.iter_mut(),
      DynamicImage::ImageRgba8(img_buf) => img_buf.iter_mut(),
      _ => bail!("invalid color format"),
    };
    let rng = &mut self.rng;
    let mut step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
    let mut data_iter = data.iter();
    let mask = (0xffu8 << self.extra.lsbs).rotate_left(self.extra.depth as u32);

    if seek > 0 {
      image_iter.nth(seek - 1);
    }

    while let Some(pixel) = image_iter.nth(step) {
      let mut bits = 0;
      for i in 0..self.extra.lsbs {
        let bit = match data_iter.next() {
          Some(bit) => bit,
          None => {
            if i == 0 {
              return Ok(());
            } else {
              break;
            }
          }
        };
        bits = bits << 1 | (if *bit { 1 } else { 0 });
      }
      *pixel = (*pixel & mask) | (bits << self.extra.depth);
      step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
    }

    Ok(())
  }

  fn write_data(&mut self, data: &[u8]) -> Result<()> {
    ensure!(!data.is_empty(), "data is empty");
    let max_step = (self.image.width() * self.image.height() * self.image.color().channel_count() as u32 - 32) as usize
      / (data.len() << 3);

    ensure!(max_step > 0, "invalid data size");
    self.write((data.len() as u32).to_le_bytes().view_bits(), 0, 0)?;
    self.write(data.view_bits(), 32, max_step)?;

    Ok(())
  }

  fn encode_image(&mut self, image_opts: ImageOptions) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    image::codecs::png::PngEncoder::new_with_quality(&mut buffer, image_opts.png.compression, image_opts.png.filter)
      .write_image(
        self.image.as_bytes(),
        self.image.width(),
        self.image.height(),
        self.image.color(),
      )?;
    Ok(buffer)
  }
}
