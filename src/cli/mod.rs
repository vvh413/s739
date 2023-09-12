mod png;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueHint};
use clap_complete::{Generator, Shell};

use self::png::{CompressionType, FilterType};

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

  /// Secret key
  #[arg(short, long, value_hint = ValueHint::Other)]
  pub key: Option<String>,
}

#[derive(Args, Debug, Clone, Default)]
pub struct ImageOptions {
  #[command(flatten)]
  pub png: PngOptions,
  #[command(flatten)]
  pub jpeg: JpegOptions,
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
  #[arg(long, default_value_t = CompressionType::Fast)]
  pub compression: CompressionType,
  /// PNG filter type
  #[arg(long, default_value_t = FilterType::Adaptive)]
  pub filter: FilterType,
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
pub struct JpegOptions {}

impl From<JpegOptions> for s739::options::JpegOptions {
  fn from(_value: JpegOptions) -> Self {
    s739::options::JpegOptions {}
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

  /// Secret key
  #[arg(short, long, value_hint = ValueHint::Other)]
  pub key: Option<String>,
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
  clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
