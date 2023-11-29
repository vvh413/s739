use criterion::{criterion_group, criterion_main};
use s739::decode::{self, Decoder};
use s739::encode::{self, Encoder};

fn decode(c: &mut criterion::Criterion) {
  let mut group = c.benchmark_group("decode");
  for size in [16, 512, 1024, 512 * 1024, 5 * 1024 * 1024].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::new("png", size), size, |b, &size| {
      let image = image::DynamicImage::ImageRgb8(image::ImageBuffer::new(4000, 4000));
      let data = vec![0u8; size];
      let mut encoder = encode::png::PngEncoder::new(image, None).unwrap();
      encoder.write_data(&data).unwrap();
      b.iter(|| {
        let mut decoder = decode::png::PngDecoder::new(encoder.image.clone(), None).unwrap();
        decoder.read_data()
      })
    });
    group.bench_with_input(criterion::BenchmarkId::new("jpeg", size), size, |b, &size| {
      let image = image::DynamicImage::ImageRgb8(image::ImageBuffer::new(4000, 4000));
      let mut image_buffer = Vec::new();
      image::codecs::jpeg::JpegEncoder::new(&mut image_buffer)
        .encode(image.as_bytes(), image.width(), image.height(), image.color())
        .unwrap();
      let data = vec![3u8; size];
      let mut encoder = encode::jpeg::JpegEncoder::new(&image_buffer, None, None).unwrap();
      encoder.write_data(&data).unwrap();
      let image_buffer = encoder.encode_image(s739::options::ImageOptions::default()).unwrap();
      b.iter(|| {
        let mut decoder = decode::jpeg::JpegDecoder::new(&image_buffer, None, None).unwrap();
        decoder.read_data()
      })
    });
  }
  group.finish();
}

criterion_group!(benches, decode);
criterion_main!(benches);
