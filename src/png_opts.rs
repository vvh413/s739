use std::fmt::Display;

use clap::ValueEnum;
use image::codecs::png;

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum CompressionType {
  Default,
  #[default]
  Fast,
  Best,
}

impl Display for CompressionType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Default => write!(f, "default"),
      Self::Fast => write!(f, "fast"),
      Self::Best => write!(f, "best"),
    }
  }
}

impl From<CompressionType> for png::CompressionType {
  fn from(val: CompressionType) -> Self {
    match val {
      CompressionType::Default => png::CompressionType::Default,
      CompressionType::Fast => png::CompressionType::Fast,
      CompressionType::Best => png::CompressionType::Best,
    }
  }
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum FilterType {
  No,
  Sub,
  Up,
  Avg,
  Paeth,
  #[default]
  Adaptive,
}

impl Display for FilterType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::No => write!(f, "no"),
      Self::Sub => write!(f, "sub"),
      Self::Up => write!(f, "up"),
      Self::Avg => write!(f, "avg"),
      Self::Paeth => write!(f, "paeth"),
      Self::Adaptive => write!(f, "adaptive"),
    }
  }
}

impl From<FilterType> for png::FilterType {
  fn from(value: FilterType) -> Self {
    match value {
      FilterType::No => png::FilterType::NoFilter,
      FilterType::Sub => png::FilterType::Sub,
      FilterType::Up => png::FilterType::Up,
      FilterType::Avg => png::FilterType::Avg,
      FilterType::Paeth => png::FilterType::Paeth,
      FilterType::Adaptive => png::FilterType::Adaptive,
    }
  }
}
