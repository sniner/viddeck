# VidDeck

VidDeck is a fast, lightweight, and modern web-based video library viewer and manager written in Rust. It automatically scans a given directory for video files, extracts thumbnails and metadata (such as resolution, framerate, codec, and chapters) using `ffprobe`/`ffmpeg`, and serves them in a responsive, modern web interface.

## Features

- **Fast scanning**: Blistering fast directory scanning and caching of metadata.
- **Modern UI**: A responsive, dark-mode first design.
- **Live Search**: Instant client-side filtering by video title or path.
- **Smart Thumbnails**: Dynamic thumbnail generation based on chapters, scrub position (percent or seconds), and video size.
- **System Integration**: Open files or their containing directories directly in your default system file explorer or media player.
- **Inline Playing**: Stream and play videos directly in your browser.
- **File Management**: Rename video files directly from the UI.

## Prerequisites

Video metadata extraction and thumbnail generation depend on FFmpeg. You must have `ffmpeg` and `ffprobe` installed and available in your system's `PATH`.

- **Ubuntu/Debian**: `sudo apt install ffmpeg`
- **Arch Linux**: `sudo pacman -S ffmpeg`
- **macOS (Homebrew)**: `brew install ffmpeg`

## Installation

You will need the [Rust toolchain](https://rustup.rs/) installed.

Clone the repository and build using Cargo:

```bash
cargo build --release
```

The compiled binary will be available at `./target/release/viddeck`.

## Usage

Start VidDeck by providing the path to a directory containing video files.

```bash
# Scan the current directory
viddeck .

# Scan a specific directory
viddeck /path/to/my/videos

# Bind to a specific host and port
viddeck /path/to/my/videos --host 127.0.0.1 --port 8080
```

Once running, VidDeck will output the URL where the web interface is accessible (e.g., `http://127.0.0.1:8765`). Open this URL in your browser.

*Note: The first time you launch VidDeck for a large directory, it will take some time to extract metadata (duration, resolution, chapters) for all videos via `ffprobe`. You can watch the progress live in your browser.*

## License

This project is licensed under the BSD 3-Clause License. See the [LICENSE](LICENSE) file for details.
