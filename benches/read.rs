use bitvec::view::AsMutBits;
use criterion::{criterion_group, criterion_main};
use s739::decode;

fn read(c: &mut criterion::Criterion) {
  let mut group = c.benchmark_group("read");
  for size in [16, 16 * 1024, 512 * 1024, 4 * 1024 * 1024, 8_000_000].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::new("buf", size), size, |b, &size| {
      let image = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000));
      let mut buf = vec![0u8; size];
      b.iter(|| {
        decode::read(&image, buf.as_mut_bits(), 0);
      })
    });
  }
  group.finish();
}

criterion_group!(benches, read);
criterion_main!(benches);
