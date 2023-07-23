use anyhow::Result;
use clap::Parser;
use s739::cli::{Cli, Command};
use s739::decode::decode;
use s739::encode::encode;

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Command::Encode(args) => encode(args)?,
    Command::Decode(args) => decode(args)?,
  }

  Ok(())
}
