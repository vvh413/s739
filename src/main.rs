use std::fs::File;
use std::io::{BufReader, Read, Write};

use anyhow::Result;
use bitvec::prelude::*;
use clap::Parser;
use cli::{Cli, Command, DecodeArgs, EncodeArgs};
use image::DynamicImage;

mod cli;

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

fn read(image: &DynamicImage, size: usize, seek: usize) -> BitVec<u8> {
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

fn encode(args: EncodeArgs) -> Result<()> {
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

fn decode(args: DecodeArgs) -> Result<()> {
  let DecodeArgs { input, file } = args;
  let input_image = image::open(input)?;

  let size: usize = read(&input_image, 4, 0).load_le();
  assert!(
    input_image.width() * input_image.height() * 3 > (size << 3) as u32,
    "invalid data size"
  );

  let data = read(&input_image, size, 4);

  match file {
    Some(file) => File::create(file)?.write_all(data.as_raw_slice()),
    None => std::io::stdout().write_all(data.as_raw_slice()),
  }?;

  Ok(())
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Command::Encode(args) => encode(args)?,
    Command::Decode(args) => decode(args)?,
  }

  Ok(())
}
