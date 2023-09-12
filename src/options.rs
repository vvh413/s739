use image::codecs::png;

#[derive(Debug, Clone, Default)]
pub struct ImageOptions {
  pub png: PngOptions,
  pub jpeg: JpegOptions,
}

#[derive(Debug, Clone, Default)]
pub struct PngOptions {
  pub compression: png::CompressionType,
  pub filter: png::FilterType,
}

#[derive(Debug, Clone, Default)]
pub struct JpegOptions {}
