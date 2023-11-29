#[cfg(feature = "cli")]
mod cli;

use std::fs::File;
use std::io::{BufReader, Read, Write};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::{print_completions, Cli, Command, DecodeArgs, EncodeArgs};
use s739::decode::new_decoder;
use s739::encode::new_encoder;

fn decode(args: DecodeArgs) -> Result<()> {
  let DecodeArgs {
    input,
    file,
    key,
    jpeg_comp,
  } = args;

  let mut decoder = new_decoder(input, key, jpeg_comp)?;
  let data = decoder.read_data()?;

  match file {
    Some(file) => std::fs::write(file, data),
    None => std::io::stdout().write_all(&data),
  }?;

  Ok(())
}

fn read_data(data: cli::Data) -> Result<Vec<u8>> {
  let mut buf = Vec::new();
  match (data.text, data.file, data.stdin) {
    (Some(text), _, _) => {
      buf = text.as_bytes().to_vec();
    }
    (_, Some(file), _) => {
      let _ = BufReader::new(File::open(file)?).read_to_end(&mut buf)?;
    }
    (_, _, true) => {
      let _ = std::io::stdin().read_to_end(&mut buf)?;
    }
    _ => unreachable!(),
  };
  Ok(buf)
}

fn encode(args: EncodeArgs) -> Result<()> {
  let EncodeArgs {
    input,
    output,
    data,
    image_opts,
    key,
    jpeg_comp,
  } = args;

  let mut encoder = new_encoder(input, key, jpeg_comp)?;
  let data = read_data(data)?;
  encoder.write_data(&data)?;
  let buffer = encoder.encode_image(image_opts.into())?;
  std::fs::write(output, buffer)?;

  Ok(())
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Command::Encode(args) => encode(args)?,
    Command::Decode(args) => decode(args)?,
    Command::Generate { shell } => print_completions(shell, &mut Cli::command_for_update()),
  }

  Ok(())
}
