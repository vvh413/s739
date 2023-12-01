mod jpeg;
mod png;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueHint};
use clap_complete::{Generator, Shell};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// Encode data to image
  Encode(EncodeArgs),
  /// Decode data from image
  Decode(DecodeArgs),
  /// Generate shell completions
  Generate { shell: Shell },
}

#[derive(Args, Debug, Clone)]
pub struct ExtraArgs {
  /// Secret key
  #[arg(short, long, value_hint = ValueHint::Other)]
  key: Option<String>,
  /// JPEG component index
  #[arg(long)]
  jpeg_comp: Option<u8>,
  /// Skip some DCT coefs for JPEG
  #[arg(long)]
  adaptive: bool,
  /// Depth (least bit to use)
  #[arg(long, default_value_t = 0, value_parser = 0..=7)]
  depth: i64,
  /// Number of bits to use
  #[arg(long, default_value_t = 1, value_parser = 1..=8)]
  lsbs: i64,
}

impl From<ExtraArgs> for s739::options::ExtraArgs {
  fn from(value: ExtraArgs) -> Self {
    Self {
      key: value.key,
      jpeg_comp: value.jpeg_comp,
      adaptive: value.adaptive,
      depth: value.depth as usize,
      lsbs: value.lsbs as usize,
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct EncodeArgs {
  /// Input file
  #[arg(short, long, value_hint = ValueHint::FilePath)]
  pub input: PathBuf,
  /// Output file
  #[arg(short, long, value_hint = ValueHint::FilePath)]
  pub output: PathBuf,
  #[command(flatten)]
  pub image_opts: ImageOptions,
  #[command(flatten)]
  pub data: Data,
  #[command(flatten)]
  pub extra_args: ExtraArgs,
}

#[derive(Args, Debug, Clone, Default)]
pub struct ImageOptions {
  #[command(flatten)]
  png: PngOptions,
  #[command(flatten)]
  jpeg: JpegOptions,
}

impl From<ImageOptions> for s739::options::ImageOptions {
  fn from(value: ImageOptions) -> Self {
    s739::options::ImageOptions {
      png: value.png.into(),
      jpeg: value.jpeg.into(),
    }
  }
}

#[derive(Args, Debug, Clone, Default)]
pub struct PngOptions {
  /// PNG compression type
  #[arg(long = "png-compression", default_value_t = png::CompressionType::Fast)]
  compression: png::CompressionType,
  /// PNG filter type
  #[arg(long = "png-filter", default_value_t = png::FilterType::Adaptive)]
  filter: png::FilterType,
}

impl From<PngOptions> for s739::options::PngOptions {
  fn from(value: PngOptions) -> Self {
    s739::options::PngOptions {
      compression: value.compression.into(),
      filter: value.filter.into(),
    }
  }
}

#[derive(Args, Debug, Clone, Default)]
pub struct JpegOptions {
  /// MozJPEG compression profile
  #[arg(long = "jpeg-compress-profile", default_value_t = jpeg::CompressProfile::Max)]
  compress_profile: jpeg::CompressProfile,
}

impl From<JpegOptions> for s739::options::JpegOptions {
  fn from(value: JpegOptions) -> Self {
    s739::options::JpegOptions {
      compress_profile: value.compress_profile.into(),
    }
  }
}

#[derive(Args, Debug, Clone)]
#[group(required = true, multiple = false)]
pub struct Data {
  /// Encode plain text data
  #[arg(short, long, value_hint = ValueHint::Other)]
  pub text: Option<String>,
  /// Encode file
  #[arg(short, long, value_hint = ValueHint::FilePath)]
  pub file: Option<PathBuf>,
  /// Read data from stdin
  #[arg(short, long)]
  pub stdin: bool,
}

#[derive(Args, Debug, Clone)]
pub struct DecodeArgs {
  /// Input file
  #[arg(short, long, value_hint = ValueHint::FilePath)]
  pub input: PathBuf,
  /// Write data to file
  #[arg(short, long, value_hint = ValueHint::FilePath)]
  pub file: Option<PathBuf>,
  #[command(flatten)]
  pub extra_args: ExtraArgs,
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
  clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
