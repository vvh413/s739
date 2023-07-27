use std::fs::File;
use std::io::Write;

use anyhow::Result;
use bitvec::prelude::*;
use image::DynamicImage;

use crate::cli::DecodeArgs;

pub fn read(image: &DynamicImage, buf: &mut BitSlice<u8>, seek: usize) {
  match image {
    DynamicImage::ImageRgb8(img_buf) => img_buf.iter(),
    DynamicImage::ImageRgba8(img_buf) => img_buf.iter(),
    _ => panic!("invalid color format"),
  }
  .skip(seek << 3)
  .zip(buf.iter_mut())
  .for_each(|(pixel, mut bit)| bit.set((pixel & 0x1) == 1))
}

pub fn decode(args: DecodeArgs) -> Result<()> {
  let DecodeArgs { input, file } = args;
  let input_image = image::open(input)?;

  let size = bits![mut u8, Lsb0; 0u8; 32];
  read(&input_image, size, 0);
  let size: usize = size.load();
  assert!(
    input_image.width() * input_image.height() * input_image.color().channel_count() as u32 > (size << 3) as u32,
    "invalid data size"
  );

  let mut data = vec![0u8; size];
  read(&input_image, data.view_bits_mut(), 4);

  match file {
    Some(file) => File::create(file)?.write_all(&data),
    None => std::io::stdout().write_all(&data),
  }?;

  Ok(())
}
