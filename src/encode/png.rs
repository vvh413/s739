use crate::options::{ExtraArgs, ImageOptions};
use crate::utils;
use anyhow::{bail, ensure, Result};
use bitvec::prelude::*;
use image::{DynamicImage, ImageEncoder};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;

use super::Encoder;

pub struct PngEncoder {
    pub image: DynamicImage,
    extra: ExtraArgs,
}

impl PngEncoder {
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

impl Encoder for PngEncoder {
    fn total_size(&self) -> usize {
        (self.image.width() as usize
            * self.image.height() as usize
            * self.image.color().channel_count() as usize
            - 32)
            * self.extra().bits
    }

    fn extra(&self) -> ExtraArgs {
        self.extra.clone()
    }

    fn write(&mut self, data: &BitSlice<u8>, seek: usize, max_step: usize) -> Result<()> {
        let mut image_iter = match &mut self.image {
            DynamicImage::ImageRgb8(img_buf) => img_buf.iter_mut(),
            DynamicImage::ImageRgba8(img_buf) => img_buf.iter_mut(),
            _ => bail!("invalid color format"),
        };

        let mut rng = ChaCha20Rng::from_seed(Seeder::from(self.extra.key.clone()).make_seed());
        let mut data_iter = data.iter();
        let mask = u8::MAX
            .checked_shl(self.extra.bits as u32)
            .unwrap_or(0)
            .rotate_left(self.extra.depth as u32);

        if seek > 0 {
            image_iter.nth(seek - 1);
        }

        while let Some(pixel) = image_iter.nth(utils::iter::rand_step(&mut rng, max_step)) {
            let bits: u8 = match utils::iter::get_n_bits(&mut data_iter, self.extra.bits) {
                Ok(bits) => bits,
                Err(_) => return Ok(()),
            };
            *pixel = (*pixel & mask) | (bits << self.extra.depth);
        }

        if data_iter.next().is_some() {
            bail!("image ended but data not");
        }
        Ok(())
    }

    fn encode_image(&self, image_opts: ImageOptions) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        image::codecs::png::PngEncoder::new_with_quality(
            &mut buffer,
            image_opts.png.compression,
            image_opts.png.filter,
        )
        .write_image(
            self.image.as_bytes(),
            self.image.width(),
            self.image.height(),
            self.image.color().into(),
        )?;
        Ok(buffer)
    }
}
