use std::fs::File;
use std::io::{BufReader, Read};

use crate::cli::EncodeArgs;
use anyhow::Result;
use bitvec::prelude::*;
use image::DynamicImage;

fn write(image: &mut DynamicImage, data: &BitSlice<u8, Lsb0>, seek: usize) {
  match image {
    DynamicImage::ImageRgb8(img_buf) => img_buf.iter_mut(),
    DynamicImage::ImageRgba8(img_buf) => img_buf.iter_mut(),
    _ => panic!("invalid color format"),
  }
  .skip(seek << 3)
  .zip(data.iter())
  .for_each(|(pixel, bit)| {
    pixel.view_bits_mut::<Lsb0>().set(0, *bit);
  })
}

pub fn encode(args: EncodeArgs) -> Result<()> {
  let EncodeArgs { input, output, data } = args;

  let mut input_image = image::open(input)?;

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

  assert!(
    input_image.width() * input_image.height() * 3 > (buf.len() << 3) as u32,
    "invalid data size"
  );

  write(&mut input_image, (buf.len() as u32).to_le_bytes().view_bits(), 0);
  write(&mut input_image, buf.view_bits(), 4);

  input_image.save(output)?;

  println!("done");
  Ok(())
}
