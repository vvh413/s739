use criterion::{criterion_group, criterion_main};
use s739::encode::{self, Encoder};
use s739::options::ExtraArgs;

fn encode(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("encode");
    for size in [16, 512, 1024, 512 * 1024, 5 * 1024 * 1024].iter() {
        group.throughput(criterion::Throughput::Bytes(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::new("png", size),
            size,
            |b, &size| {
                let image = image::DynamicImage::ImageRgb8(image::ImageBuffer::new(4000, 4000));
                let data = vec![0u8; size];
                b.iter(|| {
                    let mut encoder =
                        encode::png::PngEncoder::new(image.clone(), ExtraArgs::default()).unwrap();
                    encoder.write_data(&data).unwrap();
                    encoder.encode_image(s739::options::ImageOptions::default())
                })
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::new("jpeg", size),
            size,
            |b, &size| {
                let image = image::DynamicImage::ImageRgb8(image::ImageBuffer::new(4000, 4000));
                let mut image_buffer = Vec::new();
                image::codecs::jpeg::JpegEncoder::new(&mut image_buffer)
                    .encode(
                        image.as_bytes(),
                        image.width(),
                        image.height(),
                        image.color(),
                    )
                    .unwrap();
                let data = vec![3u8; size];
                b.iter(|| {
                    let mut encoder =
                        encode::jpeg::JpegEncoder::new(&image_buffer, ExtraArgs::default())
                            .unwrap();
                    encoder.write_data(&data).unwrap();
                    encoder.encode_image(s739::options::ImageOptions::default())
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, encode);
criterion_main!(benches);
