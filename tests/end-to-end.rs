use std::io::Read;

use anyhow::Result;
use s739::cli::{Data, DecodeArgs, EncodeArgs, ImageOpts};
use s739::{decode, encode};

#[test]
fn png() -> Result<()> {
  image::DynamicImage::ImageRgb8(image::ImageBuffer::new(127, 127)).save("/tmp/s739_in.png")?;
  let enc_args = EncodeArgs {
    input: "/tmp/s739_in.png".into(),
    output: "/tmp/s739_out_default.png".into(),
    image_opts: ImageOpts::default(),
    data: Data {
      text: Some("e2e test".repeat(3)),
      file: None,
      stdin: false,
    },
    key: None,
  };
  encode::encode(enc_args.clone())?;

  let dec_args = DecodeArgs {
    input: "/tmp/s739_out_default.png".into(),
    file: Some("/tmp/s739_result.txt".into()),
    key: None,
  };
  decode::decode(dec_args.clone())?;

  let mut result = String::new();
  std::fs::File::open(dec_args.file.unwrap())?.read_to_string(&mut result)?;

  assert_eq!(result.trim_end(), enc_args.data.text.unwrap());
  Ok(())
}

#[test]
fn jpeg() -> Result<()> {
  image::DynamicImage::ImageRgb8(image::ImageBuffer::new(127, 127)).save("/tmp/s739_in.jpg")?;
  let enc_args = EncodeArgs {
    input: "/tmp/s739_in.jpg".into(),
    output: "/tmp/s739_out_default.jpg".into(),
    image_opts: ImageOpts::default(),
    data: Data {
      text: Some("e2e test".repeat(3)),
      file: None,
      stdin: false,
    },
    key: None,
  };
  encode::encode(enc_args.clone())?;

  let dec_args = DecodeArgs {
    input: "/tmp/s739_out_default.jpg".into(),
    file: Some("/tmp/s739_result.txt".into()),
    key: None,
  };
  decode::decode(dec_args.clone())?;

  let mut result = String::new();
  std::fs::File::open(dec_args.file.unwrap())?.read_to_string(&mut result)?;
  println!("{result}");
  assert_eq!(result.trim_end(), enc_args.data.text.unwrap());
  Ok(())
}
