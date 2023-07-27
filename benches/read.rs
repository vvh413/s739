use criterion::{criterion_group, criterion_main};
use s739::decode;

fn read(c: &mut criterion::Criterion) {
  let mut group = c.benchmark_group("read");
  for size in [16 * 1024, 512 * 1024, 4 * 1024 * 1024, 8_000_000].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::new("collect", size), size, |b, &size| {
      let image = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000));
      b.iter(|| {
        let _data = decode::read(&image, size, 0).as_raw_slice();
      })
    });
  }
  group.finish();
}

criterion_group!(benches, read);
criterion_main!(benches);
