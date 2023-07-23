#![feature(test)]

use bitvec::view::AsBits;
use image::DynamicImage;
use s739::encode;
extern crate test;

fn prepare(width: u32, height: u32, data_size: usize) -> (DynamicImage, Vec<u8>) {
  let image = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(width, height));
  let data = vec![0u8; data_size];
  (image, data)
}

#[bench]
fn write_small(b: &mut test::Bencher) {
  let (mut image, data) = prepare(4000, 4000, 16 * 1024);
  b.iter(|| encode::write(&mut image, data.as_bits(), 0))
}

#[bench]
fn write_medium(b: &mut test::Bencher) {
  let (mut image, data) = prepare(4000, 4000, 512 * 1024);
  b.iter(|| encode::write(&mut image, data.as_bits(), 0))
}

#[bench]
fn write_full(b: &mut test::Bencher) {
  let (mut image, data) = prepare(4000, 4000, (4000 * 4000 * 4) >> 3);
  b.iter(|| encode::write(&mut image, data.as_bits(), 0))
}
