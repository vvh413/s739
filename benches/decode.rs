use criterion::{criterion_group, criterion_main};
use s739::{decode, encode};

fn decode(c: &mut criterion::Criterion) {
  let mut group = c.benchmark_group("decode");
  for size in [16, 512, 1024, 512 * 1024, 7 * 1024 * 1024].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::new("default", size), size, |b, &size| {
      let mut image = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000));
      let data = vec![0u8; size];
      let mut encoder = encode::Encoder::new(&mut image, &data, None).unwrap();
      encoder.write_data().unwrap();
      b.iter(|| {
        let mut decoder = decode::Decoder::new(&image, None).unwrap();
        decoder.read_data()
      })
    });
  }
  group.finish();
}

criterion_group!(benches, decode);
criterion_main!(benches);
