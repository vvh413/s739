use std::io::Read;

use anyhow::Result;
use s739::cli::{Data, DecodeArgs, EncodeArgs, PngOpts};
use s739::{decode, encode};

#[test]
fn default() -> Result<()> {
  image::DynamicImage::ImageRgba8(image::ImageBuffer::new(128, 128)).save("/tmp/s739_in.png")?;
  let enc_args = EncodeArgs {
    input: "/tmp/s739_in.png".into(),
    output: "/tmp/s739_out_default.png".into(),
    png_opts: PngOpts::default(),
    data: Data {
      text: Some("e2e test".repeat(100)),
      file: None,
      stdin: false,
    },
  };
  encode::encode(enc_args.clone())?;

  let dec_args = DecodeArgs {
    input: "/tmp/s739_out_default.png".into(),
    file: Some("/tmp/s739_result.txt".into()),
  };
  decode::decode(dec_args.clone())?;

  let mut result = String::new();
  std::fs::File::open(dec_args.file.unwrap())?.read_to_string(&mut result)?;

  assert_eq!(result.trim_end(), enc_args.data.text.unwrap());
  Ok(())
}
