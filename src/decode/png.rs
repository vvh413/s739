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
  depth: usize,
}

impl PngDecoder {
  pub fn new(image: DynamicImage, extra_args: ExtraArgs) -> Result<Self> {
    Ok(Self {
      image,
      rng: ChaCha20Rng::from_seed(Seeder::from(extra_args.key).make_seed()),
      depth: extra_args.depth,
    })
  }
}

impl Decoder for PngDecoder {
  fn read(&mut self, buf: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
    let mut image_iter = match &self.image {
      DynamicImage::ImageRgb8(img_buf) => img_buf.iter(),
      DynamicImage::ImageRgba8(img_buf) => img_buf.iter(),
      _ => bail!("invalid color format"),
    };
    let rng = &mut self.rng;

    if seek > 0 {
      image_iter.nth(seek - 1);
    }
    for mut bit in buf.iter_mut() {
      let step = if max_step > 1 { rng.gen_range(0..max_step) } else { 0 };
      let pixel = match image_iter.nth(step) {
        Some(pixel) => pixel,
        None => bail!("read: image ended, but data not"),
      };
      bit.set((pixel >> self.depth & 1) == 1);
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
