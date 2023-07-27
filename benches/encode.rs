use criterion::{criterion_group, criterion_main};
use s739::cli::{Data, EncodeArgs, PngOpts};
use s739::encode;

fn prepare(size: usize) -> EncodeArgs {
  std::fs::write("/tmp/s739_encode_bench.txt", "z".repeat(size)).unwrap();
  EncodeArgs {
    input: "/tmp/s739_encode_test.png".into(),
    output: "/tmp/s739.png".into(),
    png_opts: PngOpts::default(),
    data: Data {
      text: None,
      file: Some("/tmp/s739_encode_bench.txt".into()),
      stdin: false,
    },
  }
}

fn encode(c: &mut criterion::Criterion) {
  image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000))
    .save("/tmp/s739_encode_test.png")
    .unwrap();
  let mut group = c.benchmark_group("encode");
  for size in [16, 512, 1024, 512 * 1024, 7 * 1024 * 1024].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::new("default", size), size, |b, &size| {
      let args = prepare(size);
      b.iter(|| encode::encode(args.clone()).unwrap())
    });
  }
  group.finish();
}

criterion_group!(benches, encode);
criterion_main!(benches);
