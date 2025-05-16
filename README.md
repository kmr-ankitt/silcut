# Silcut

**Silcut** is a blazingly fast tool designed to quickly remove silent sections from audio and video files.

## Features

- Blazingly fast silence detection and removal
- Supports both audio and video files
- Easy to use command-line interface
- Fast and efficient using FFmpeg

---

## Requirements

- Rust
- [FFmpeg](https://ffmpeg.org/download.html) must be installed and accessible in your `PATH`.

---

## Installation

### Install using cargo

```bash
cargo install silcut
```

### Install from source (requires Rust & FFmpeg)

```bash
git clone https://github.com/kmr-ankitt/silcut.git
cd silcut
cargo build --release
````

Then place the binary into your local bin:

```bash
sudo cp ./target/release/silcut /usr/local/bin/
```


### Prebuilt binary

Download the binary from [Releases](https://github.com/kmr-ankitt/silcut/releases).

---

## Usage

```bash
silcut --input myaudio.mp3 --output ./dist
```

This will:

1. Detect and keep important segments.
2. Save those segments.
3. Merge them into `./dist/myaudio.mp3`.

### Command-line Options

You can view all available options with:

```bash
silcut -h
```

```
Usage: silcut [OPTIONS] --file-path <FILE_PATH>

Options:
  -i, --file-path <FILE_PATH>                Path to the input file (required)
  -o, --out-path <OUT_PATH>                  Output directory [default: .]
  -s, --silence <SILENCE>                    Silence threshold in dB [default: -30]
  -d, --minimum-silence-duration <SECONDS>   Minimum silence duration in seconds [default: 0.5]
  -h, --help                                Print help
  -V, --version                             Print version
```

#### Example

Remove silence from a file with custom settings:

```bash
silcut -i input.mp4 -o ./output -s -35 -d 1.0
```

This sets the silence threshold to -35 dB and minimum silence duration to 1 second.
