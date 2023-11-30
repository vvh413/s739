use anyhow::Result;
use s739::decode::new_decoder;
use s739::encode::new_encoder;
use s739::options::{ExtraArgs, ImageOptions};

fn e2e(ext: &str) -> Result<()> {
  let in_path = format!("/tmp/s739_in.{ext}");
  let out_path = format!("/tmp/s739_out.{ext}");
  let data = format!("s739 test {ext}").repeat(3).into_bytes();

  println!("encoding: {ext}");
  image::DynamicImage::ImageRgb8(image::ImageBuffer::new(127, 127)).save(in_path.clone())?;
  let mut encoder = new_encoder(in_path.into(), ExtraArgs::default())?;
  encoder.write_data(&data)?;
  let buffer = encoder.encode_image(ImageOptions::default())?;
  std::fs::write(out_path.clone(), buffer)?;

  println!("decoding: {ext}");
  let mut decoder = new_decoder(out_path.into(), ExtraArgs::default())?;
  let decoded_data = decoder.read_data()?;
  println!("done: {ext}");

  assert_eq!(decoded_data, data);
  Ok(())
}

#[test]
fn png() -> Result<()> {
  e2e("png")?;
  Ok(())
}

#[test]
fn jpeg() -> Result<()> {
  e2e("jpeg")?;
  Ok(())
}
