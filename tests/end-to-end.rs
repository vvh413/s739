use anyhow::Result;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use s739::decode::new_decoder;
use s739::encode::new_encoder;
use s739::options::{ExtraArgs, ImageOptions};

fn rand_string(size: usize) -> String {
  thread_rng()
    .sample_iter(Alphanumeric)
    .take(size)
    .map(char::from)
    .collect()
}

fn e2e(ext: &str, image_size: (u32, u32), data_size: usize, extra: ExtraArgs) -> Result<()> {
  println!("--- {image_size:?} {data_size} {extra:?}");
  let in_path = format!("/tmp/s739_in_{image_size:?}_{data_size}_{extra:?}.{ext}");
  let out_path = format!("/tmp/s739_out_{image_size:?}_{data_size}_{extra:?}.{ext}");
  let data = rand_string(data_size).into_bytes();

  println!("encoding");
  image::DynamicImage::ImageRgb8(image::ImageBuffer::new(image_size.0, image_size.1)).save(in_path.clone())?;
  let mut encoder = new_encoder(in_path.into(), extra.clone())?;
  println!("image_size: {}", encoder.total_size());
  encoder.write_data(&data)?;
  let buffer = encoder.encode_image(ImageOptions::default())?;
  std::fs::write(out_path.clone(), buffer)?;

  println!("decoding");
  let mut decoder = new_decoder(out_path.clone().into(), extra.clone())?;
  let decoded_data = decoder.read_data()?;
  println!("done: {ext}");
  assert_eq!(decoded_data, data);

  if extra.key.is_some() {
    println!("decoding wrong key");
    let mut wrong_extra = extra;
    wrong_extra.key = None;
    let mut decoder = new_decoder(out_path.into(), wrong_extra)?;
    let decoded_data = decoder.read_data()?;
    println!("done");
    assert_ne!(decoded_data, data);
  }

  Ok(())
}

#[test]
fn png_default() -> Result<()> {
  e2e("png", (128, 128), 128, ExtraArgs::default())?;
  Ok(())
}

#[test]
fn jpeg_default() -> Result<()> {
  e2e("jpg", (128, 128), 128, ExtraArgs::default())?;
  Ok(())
}

#[test]
fn png_key() -> Result<()> {
  e2e(
    "png",
    (128, 128),
    128,
    ExtraArgs {
      key: Some("some key".to_string()),
      ..Default::default()
    },
  )?;
  Ok(())
}

#[test]
fn jpeg_key() -> Result<()> {
  e2e(
    "jpg",
    (128, 128),
    128,
    ExtraArgs {
      key: Some("some key".to_string()),
      ..Default::default()
    },
  )?;
  Ok(())
}

#[test]
fn png_depth() -> Result<()> {
  for depth in 0..=7 {
    e2e(
      "png",
      (128, 128),
      128,
      ExtraArgs {
        depth,
        ..Default::default()
      },
    )?;
  }
  Ok(())
}

#[test]
fn jpeg_depth() -> Result<()> {
  for depth in 0..=7 {
    e2e(
      "jpg",
      (128, 128),
      128,
      ExtraArgs {
        depth,
        ..Default::default()
      },
    )?;
  }
  Ok(())
}

#[test]
fn png_lsbs() -> Result<()> {
  for lsbs in 1..=8 {
    e2e(
      "png",
      (128, 128),
      13,
      ExtraArgs {
        lsbs,
        ..Default::default()
      },
    )?;
  }
  Ok(())
}

#[test]
fn jpeg_lsbs() -> Result<()> {
  for lsbs in 1..=8 {
    e2e(
      "jpg",
      (128, 128),
      128,
      ExtraArgs {
        lsbs,
        ..Default::default()
      },
    )?;
  }
  Ok(())
}

#[test]
fn png_wrong_depth_and_lsbs() {
  let result = e2e(
    "png",
    (128, 128),
    128,
    ExtraArgs {
      depth: 2,
      lsbs: 7,
      ..Default::default()
    },
  );
  assert!(result.is_err());
}

#[test]
fn jpeg_wrong_depth_and_lsbs() {
  let result = e2e(
    "jpg",
    (128, 128),
    128,
    ExtraArgs {
      depth: 7,
      lsbs: 2,
      ..Default::default()
    },
  );
  assert!(result.is_err());
}
#[test]
fn png_full() -> Result<()> {
  let total_size = 128 * 128 * 3 - 32;
  e2e("png", (128, 128), total_size >> 3, ExtraArgs::default())?;
  e2e(
    "png",
    (128, 128),
    total_size,
    ExtraArgs {
      lsbs: 8,
      ..Default::default()
    },
  )?;
  Ok(())
}

#[test]
fn jpeg_full() -> Result<()> {
  let total_size = 128 * 128 * 3 - 32;
  e2e("jpg", (128, 128), total_size >> 3, ExtraArgs::default())?;
  e2e(
    "jpg",
    (128, 128),
    total_size,
    ExtraArgs {
      lsbs: 8,
      ..Default::default()
    },
  )?;
  Ok(())
}

#[test]
fn jpeg_components() -> Result<()> {
  for comp in 0..3 {
    e2e(
      "jpg",
      (128, 128),
      128,
      ExtraArgs {
        jpeg_comp: Some(comp),
        ..Default::default()
      },
    )?;
  }
  Ok(())
}
