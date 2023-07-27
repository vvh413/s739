use bitvec::view::AsBits;
use criterion::{criterion_group, criterion_main};
use s739::encode;

fn write(c: &mut criterion::Criterion) {
  let mut group = c.benchmark_group("write");
  for size in [16, 16 * 1024, 512 * 1024, 4 * 1024 * 1024, 8_000_000].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::from_parameter(size), size, |b, &size| {
      let mut image = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000));
      let data = vec![0u8; size];
      b.iter(|| encode::write(&mut image, data.as_bits(), 0))
    });
  }
  group.finish();
}

criterion_group!(benches, write);
criterion_main!(benches);
