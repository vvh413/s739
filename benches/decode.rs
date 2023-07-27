use criterion::{criterion_group, criterion_main};
use s739::cli::{Data, DecodeArgs, EncodeArgs, PngOpts};
use s739::{decode, encode};

fn prepare(size: usize) -> DecodeArgs {
  std::fs::write("/tmp/s739_decode_bench.txt", "z".repeat(size)).unwrap();
  let args = EncodeArgs {
    input: "/tmp/s739_decode_test.png".into(),
    output: format!("/tmp/s739_out_{size}.png").into(),
    png_opts: PngOpts::default(),
    data: Data {
      text: None,
      file: Some("/tmp/s739_decode_bench.txt".into()),
      stdin: false,
    },
  };
  encode::encode(args).unwrap();
  DecodeArgs {
    input: format!("/tmp/s739_out_{size}.png").into(),
    file: Some("/dev/null".into()),
  }
}

fn decode(c: &mut criterion::Criterion) {
  image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000))
    .save("/tmp/s739_decode_test.png")
    .unwrap();
  let mut group = c.benchmark_group("decode");
  for size in [16, 512, 1024, 512 * 1024, 7 * 1024 * 1024].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::new("default", size), size, |b, &size| {
      let args = prepare(size);
      b.iter(|| decode::decode(args.clone()).unwrap())
    });
  }
  group.finish();
}

criterion_group!(benches, decode);
criterion_main!(benches);
