use anyhow::Result;
use bitvec::prelude::*;
use clap::{Args, Parser, Subcommand};
use image::{DynamicImage, Pixel};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
  /// Encode data to image
  Encode(EncodeArgs),

  /// Decode data from image
  Decode(DecodeArgs),
}

#[derive(Args, Debug)]
struct EncodeArgs {
  /// Input file
  #[arg(short, long)]
  input: String,

  /// Output file
  #[arg(short, long)]
  output: String,

  /// data to encode
  #[arg(short, long)]
  data: String,
}

#[derive(Args, Debug)]
struct DecodeArgs {
  /// Input file
  #[arg(short, long)]
  input: String,
}

fn write(image: &mut DynamicImage, data: &BitSlice<u8, Lsb0>, seek: usize) {
  image
    .as_mut_rgb8()
    .unwrap()
    .pixels_mut()
    .flat_map(|pix| pix.channels_mut())
    .skip(seek << 3)
    .zip(data.iter())
    .for_each(|(pixel_channel, bit)| {
      pixel_channel.view_bits_mut::<Lsb0>().set(0, *bit);
    });
}

fn read(image: &DynamicImage, size: usize, seek: usize) -> BitVec<u8> {
  image
    .as_rgb8()
    .unwrap()
    .pixels()
    .flat_map(|pix| pix.channels())
    .skip(seek << 3)
    .take(size << 3)
    .map(|pixel_channel| (pixel_channel & 0x1) == 1)
    .collect()
}

fn encode(args: EncodeArgs) -> Result<()> {
  let EncodeArgs { input, output, data } = args;
  let mut input_image = image::open(input)?;

  assert!(
    input_image.width() * input_image.height() * 3 > data.len() as u32,
    "invalid data size"
  );

  write(&mut input_image, (data.len() as u32).to_le_bytes().view_bits(), 0);
  write(&mut input_image, data.as_bytes().view_bits(), 4);

  input_image.save(output)?;

  println!("done");
  Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
  let DecodeArgs { input } = args;
  let input_image = image::open(input)?;

  let size: usize = read(&input_image, 4, 0).load_le();
  assert!(
    input_image.width() * input_image.height() * 3 > size as u32,
    "invalid data size"
  );

  let data = read(&input_image, size, 4);

  println!("{}", String::from_utf8(data.as_raw_slice().to_vec())?);

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
