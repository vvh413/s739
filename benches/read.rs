#![feature(test)]

use s739::decode;
extern crate test;

#[bench]
fn read_small(b: &mut test::Bencher) {
  let image = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000));
  b.iter(|| {
    let _data = decode::read(&image, 16 * 1024, 0).as_raw_slice();
  })
}

#[bench]
fn read_medium(b: &mut test::Bencher) {
  let image = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000));
  b.iter(|| {
    let _data = decode::read(&image, 512 * 1024, 0).as_raw_slice();
  })
}

#[bench]
fn read_full(b: &mut test::Bencher) {
  let image = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000));
  b.iter(|| {
    let _data = decode::read(&image, (4000 * 4000 * 4) >> 3, 0).as_raw_slice();
  })
}
