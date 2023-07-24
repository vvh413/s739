# s739

Steganography tool

Features:
 - Image containers (8-bit RGB/RGBA PNG). JPEG and other image formats can be used as input file, but they will be re-encoded to 8-bit RGB/RGBA PNG on the output.
 - Supports plain text, files and stdin
 - LSB algorithm
 - Shell completions

## Installation

#### Static build from releases
```sh
curl -OL https://github.com/vvh413/s739/releases/latest/download/s739 --output ~/.local/bin/
```

#### Build from source
```sh
git clone https://github.com/vvh413/s739
cd s739
cargo install --path .
```

#### Shell completions
Add the following to your shell config file (`~/.bashrc`, `~/.zshrc`, etc.):
- bash:
```sh
eval "$(s739 generate bash)"
```
- zsh:
```sh
eval "$(s739 generate zsh)"
```
- fish:
```sh
s739 generate fish | source
```
Supported shells: bash, zsh, fish, elvish, powershell.

## Usage

```sh 
$ s739 -h 
Steganography tool

Usage: s739 <COMMAND>

Commands:
  encode    Encode data to image
  decode    Decode data from image
  generate  Generate shell completions
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Encode

```sh 
$ s739 encode -h 
Encode data to image

Usage: s739 encode [OPTIONS] --input <INPUT> --output <OUTPUT> <--text <TEXT>|--file <FILE>|--stdin>

Options:
  -i, --input <INPUT>
          Input file
  -o, --output <OUTPUT>
          Output file
      --png-compression <PNG_COMPRESSION>
          PNG compression type [default: fast] [possible values: default, fast, best]
      --png-filter <PNG_FILTER>
          PNG filter type [default: adaptive] [possible values: no, sub, up, avg, paeth, adaptive]
  -t, --text <TEXT>
          Encode plain text data
  -f, --file <FILE>
          Encode file
  -s, --stdin
          Read data from stdin
  -h, --help
          Print help
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
