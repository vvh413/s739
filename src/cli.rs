use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueHint};
use clap_complete::{Generator, Shell};

use crate::png_opts::{CompressionType, FilterType};

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
  pub png_opts: PngOpts,

  #[command(flatten)]
  pub data: Data,

  /// Secret key
  #[arg(short, long, value_hint = ValueHint::Other)]
  pub key: Option<String>,
}

#[derive(Args, Debug, Clone, Default)]
pub struct PngOpts {
  /// PNG compression type
  #[arg(long, default_value_t = CompressionType::Fast)]
  pub png_compression: CompressionType,
  /// PNG filter type
  #[arg(long, default_value_t = FilterType::Adaptive)]
  pub png_filter: FilterType,
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
