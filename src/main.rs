use anyhow::Result;
use clap::{CommandFactory, Parser};
use s739::cli::{print_completions, Cli, Command};
use s739::decode::decode;
use s739::encode::encode;

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Command::Encode(args) => encode(args)?,
    Command::Decode(args) => decode(args)?,
    Command::Generate { shell } => print_completions(shell, &mut Cli::command_for_update()),
  }

  Ok(())
}
