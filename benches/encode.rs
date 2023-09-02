use criterion::{criterion_group, criterion_main};
use s739::encode::{self, Encoder};

fn encode(c: &mut criterion::Criterion) {
  let mut group = c.benchmark_group("encode");
  for size in [16, 512, 1024, 512 * 1024, 7 * 1024 * 1024].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::new("default", size), size, |b, &size| {
      let image = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000));
      let data = vec![0u8; size];
      let mut encoder = encode::png::PngEncoder::new(image, None).unwrap();
      b.iter(|| {
        encoder.write_data(&data).unwrap();
      })
    });
  }
  group.finish();
}

criterion_group!(benches, encode);
criterion_main!(benches);
