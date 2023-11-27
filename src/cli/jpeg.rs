use std::fmt::Display;

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum CompressProfile {
  #[default]
  Max,
  Fastest,
}

impl Display for CompressProfile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Max => write!(f, "max"),
      Self::Fastest => write!(f, "fastest"),
    }
  }
}

impl From<CompressProfile> for mozjpeg_sys::JINT_COMPRESS_PROFILE_VALUE {
  fn from(value: CompressProfile) -> Self {
    match value {
      CompressProfile::Max => mozjpeg_sys::JINT_COMPRESS_PROFILE_VALUE::JCP_MAX_COMPRESSION,
      CompressProfile::Fastest => mozjpeg_sys::JINT_COMPRESS_PROFILE_VALUE::JCP_FASTEST,
    }
  }
}
