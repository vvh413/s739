use derivative::Derivative;
use image::codecs::png;
use mozjpeg_sys::JINT_COMPRESS_PROFILE_VALUE;

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

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct JpegOptions {
  #[derivative(Default(value = "JINT_COMPRESS_PROFILE_VALUE::JCP_MAX_COMPRESSION"))]
  pub compress_profile: JINT_COMPRESS_PROFILE_VALUE,
}

#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct ExtraArgs {
  pub key: Option<String>,
  pub selective: bool,
  pub depth: usize,
  #[derivative(Default(value = "1"))]
  pub bits: usize,
  pub jpeg_comp: Option<u8>,
  pub max_step: Option<usize>,
}
