use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

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
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
  /// Input file
  #[arg(short, long)]
  pub input: PathBuf,

  /// Output file
  #[arg(short, long)]
  pub output: PathBuf,

  #[command(flatten)]
  pub png_opts: PngOpts,

  #[command(flatten)]
  pub data: Data,
}

#[derive(Args, Debug)]
pub struct PngOpts {
  /// PNG compression type
  #[arg(long, default_value_t = CompressionType::Fast)]
  pub png_compression: CompressionType,
  /// PNG filter type
  #[arg(long, default_value_t = FilterType::Adaptive)]
  pub png_filter: FilterType,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Data {
  /// Encode plain text data
  #[arg(short, long)]
  pub text: Option<String>,

  /// Encode file
  #[arg(short, long)]
  pub file: Option<PathBuf>,

  /// Read data from stdin
  #[arg(short, long)]
  pub stdin: bool,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
  /// Input file
  #[arg(short, long)]
  pub input: PathBuf,

  /// Write data to file
  #[arg(short, long)]
  pub file: Option<PathBuf>,
}
