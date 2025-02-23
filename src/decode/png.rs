use anyhow::{bail, ensure, Result};
use bitvec::prelude::*;
use image::DynamicImage;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use crate::options::ExtraArgs;
use crate::utils;

use super::Decoder;

pub struct PngDecoder {
    image: DynamicImage,
    extra: ExtraArgs,
}

impl PngDecoder {
    pub fn new(image: DynamicImage, extra: ExtraArgs) -> Result<Self> {
        ensure!(
            extra.depth + extra.bits <= 8,
            "invalid depth and bits: {} + {} > 8",
            extra.depth,
            extra.bits
        );
        Ok(Self { image, extra })
    }
}

impl Decoder for PngDecoder {
    fn total_size(&self) -> usize {
        (self.image.width() as usize
            * self.image.height() as usize
            * self.image.color().channel_count() as usize
            - 32)
            * self.extra().bits
    }

    fn extra(&self) -> &ExtraArgs {
        &self.extra
    }

    fn read(&self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
        let mut image_iter = match &self.image {
            DynamicImage::ImageRgb8(img_buf) => img_buf.iter(),
            DynamicImage::ImageRgba8(img_buf) => img_buf.iter(),
            _ => bail!("invalid color format"),
        };
        let mut rng = ChaCha20Rng::from_seed(Seeder::from(self.extra.key.clone()).make_seed());
        let mut data_iter = data.iter_mut();
        let mask = !u8::MAX.checked_shl(self.extra().bits as u32).unwrap_or(0);
        let shift = u8::BITS as usize - self.extra().bits;

        if seek > 0 {
            image_iter.nth(seek - 1);
        }

        while let Some(coef) = image_iter.nth(utils::iter::rand_step(&mut rng, max_step)) {
            let value = ((*coef >> self.extra().depth) & mask).reverse_bits() >> shift;
            if utils::iter::set_n_bits(value, &mut data_iter, self.extra().bits).is_err() {
                return Ok(());
            }
        }

        if data_iter.next().is_some() {
            bail!("image ended but data not");
        }
        Ok(())
    }
}
