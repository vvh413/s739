# s739

Steganography tool

Features:
 - image containers (any lossless RGB/RGBA)
 - supports as input plain text, files and stdin
 - LSB algorithm

## Installation

```sh
git clone https://github.com/vvh413/s739
cd s739
cargo install --path .
```

## Usage

```sh 
$ s739 -h 
Steganography tool

Usage: s739 <COMMAND>

Commands:
  encode  Encode data to image
  decode  Decode data from image
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Encode

```sh 
$ s739 encode -h 
Encode data to image

Usage: s739 encode --input <INPUT> --output <OUTPUT> <--text <TEXT>|--file <FILE>|--stdin>

Options:
  -i, --input <INPUT>    Input file
  -o, --output <OUTPUT>  Output file
  -t, --text <TEXT>      Encode plain text data
  -f, --file <FILE>      Encode file
  -s, --stdin            Read data from stdin
  -h, --help             Print help
```

### Decode

```sh
$ s739 decode -h
Decode data from image

Usage: s739 decode [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>  Input file
  -f, --file <FILE>    Write data to file
  -h, --help           Print help
```
