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

pub struct ExtraArgs {
  pub key: Option<String>,
  pub jpeg_comp: Option<u8>,
  pub adaptive: bool,
  pub depth: usize,
  pub lsbs: usize,
}

impl Default for ExtraArgs {
  fn default() -> Self {
    Self {
      key: None,
      jpeg_comp: None,
      adaptive: false,
      depth: 0,
      lsbs: 1,
    }
  }
}
