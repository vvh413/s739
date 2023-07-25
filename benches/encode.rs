use criterion::{criterion_group, criterion_main};
use s739::cli::{Data, EncodeArgs, PngOpts};
use s739::encode;

fn encode(c: &mut criterion::Criterion) {
  let mut group = c.benchmark_group("encode");
  for size in [16, 512, 1024, 512 * 1024].iter() {
    group.throughput(criterion::Throughput::Bytes(*size as u64));
    group.bench_with_input(criterion::BenchmarkId::from_parameter(size), size, |b, &size| {
      let args = EncodeArgs {
        input: "assets/random.png".into(),
        output: "/tmp/s739.png".into(),
        png_opts: PngOpts::default(),
        data: Data {
          text: Some(String::with_capacity(size)),
          file: None,
          stdin: false,
        },
      };
      b.iter(|| encode::encode(args.clone()))
    });
  }
  group.finish();
}

criterion_group!(benches, encode);
criterion_main!(benches);
