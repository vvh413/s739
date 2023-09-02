use criterion::{criterion_group, criterion_main};
use s739::cli::{Data, DecodeArgs, EncodeArgs, PngOpts};
use s739::{decode, encode};

fn prepare(size: usize, ext: &str) -> (EncodeArgs, DecodeArgs) {
  std::fs::write("/tmp/s739_decode_bench.txt", "z".repeat(size)).unwrap();
  let enc_args = EncodeArgs {
    input: format!("/tmp/s739_decode_test.{ext}").into(),
    output: format!("/tmp/s739_out_{size}.{ext}").into(),
    png_opts: PngOpts::default(),
    data: Data {
      text: None,
      file: Some("/tmp/s739_decode_bench.txt".into()),
      stdin: false,
    },
    key: None,
  };
  let dec_args = DecodeArgs {
    input: format!("/tmp/s739_out_{size}.{ext}").into(),
    file: Some("/dev/null".into()),
    key: None,
  };
  (enc_args, dec_args)
}

fn png(c: &mut criterion::Criterion) {
  image::DynamicImage::ImageRgba8(image::ImageBuffer::new(4000, 4000))
    .save("/tmp/s739_decode_test.png")
    .unwrap();
  let mut group = c.benchmark_group("e2e-png");
  for size in [16, 512, 1024, 512 * 1024, 7 * 1024 * 1024].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::new("default", size), size, |b, &size| {
      let (enc_args, dec_args) = prepare(size, "png");
      b.iter(|| {
        encode::encode(enc_args.clone()).unwrap();
        decode::decode(dec_args.clone()).unwrap()
      })
    });
  }
  group.finish();
}

fn jpeg(c: &mut criterion::Criterion) {
  image::DynamicImage::ImageRgb8(image::ImageBuffer::new(4000, 4000))
    .save("/tmp/s739_decode_test.jpg")
    .unwrap();
  let mut group = c.benchmark_group("e2e-jpeg");
  for size in [16, 512, 1024, 512 * 1024, 5 * 1024 * 1024].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::new("default", size), size, |b, &size| {
      let (enc_args, dec_args) = prepare(size, "jpg");
      b.iter(|| {
        encode::encode(enc_args.clone()).unwrap();
        decode::decode(dec_args.clone()).unwrap()
      })
    });
  }
  group.finish();
}

criterion_group!(benches, png, jpeg);
criterion_main!(benches);
