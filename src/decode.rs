use std::fs::File;
use std::io::Write;

use anyhow::Result;
use bitvec::prelude::*;
use image::DynamicImage;

use crate::cli::DecodeArgs;

pub fn read(image: &DynamicImage, size: usize, seek: usize) -> BitVec<u8> {
  match image {
    DynamicImage::ImageRgb8(img_buf) => img_buf.iter(),
    DynamicImage::ImageRgba8(img_buf) => img_buf.iter(),
    _ => panic!("invalid color format"),
  }
  .skip(seek << 3)
  .take(size << 3)
  .map(|pixel_channel| (pixel_channel & 0x1) == 1)
  .collect()
}

pub fn read_to(image: &DynamicImage, buf: &mut BitSlice<u8, Lsb0>, seek: usize) {
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

  let size: usize = read(&input_image, 4, 0).load_le();
  assert!(
    input_image.width() * input_image.height() * input_image.color().channel_count() as u32 > (size << 3) as u32,
    "invalid data size"
  );

  let data = read(&input_image, size, 4);

  match file {
    Some(file) => File::create(file)?.write_all(data.as_raw_slice()),
    None => std::io::stdout().write_all(data.as_raw_slice()),
  }?;

  Ok(())
}
