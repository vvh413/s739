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

#[derive(Debug, Clone)]
pub struct JpegOptions {
  pub compress_profile: mozjpeg_sys::JINT_COMPRESS_PROFILE_VALUE,
}

impl Default for JpegOptions {
  fn default() -> Self {
    Self {
      compress_profile: mozjpeg_sys::JINT_COMPRESS_PROFILE_VALUE::JCP_MAX_COMPRESSION,
    }
  }
}
