# VidDeck

VidDeck shows the videos in a directory tree as a library in the web browser:
one card per video, with a thumbnail for each chapter, the duration, resolution,
codecs and file size. Videos play in the browser or in the player of the system.
VidDeck reads the videos with FFmpeg. It keeps no database, and it changes
nothing in the directory unless you rename a file.

## Features

- **Library** of all MKV, MP4, M4V, AVI, MOV and WebM files below a directory,
  in pages of 50, with duration, file size, resolution, frame rate and the video
  and audio codecs of each video
- **Chapter thumbnails**: one per chapter, or one for a video without chapters.
  You choose where in the chapter the thumbnail is taken (percent or seconds) and
  how large it is. A click enlarges it, with a button that plays the video from
  that chapter
- **Search** by file name or path, as you type
- **Playback** in the browser or in the default player of the system. A video the
  browser cannot play, such as HEVC video or AC3 audio, is converted while it
  plays, with a hardware encoder where one is available
- **Open the folder** of a video in the file manager
- **Rename** a video, or move it into a subdirectory of the library
- **Follow changes** with `--watch`: new, changed and removed files show up in
  open browser windows without a reload
- Light and dark theme, following the system setting

## Installation

### With Homebrew (macOS)

```bash
brew install sniner/tap/viddeck
```

Homebrew installs FFmpeg along with it.

### Pre-built binaries

Every [release](https://github.com/sniner/viddeck/releases) includes:

- `viddeck-vX.Y.Z-x86_64-linux-musl` / `-aarch64-linux-musl`: Linux, statically linked
- `viddeck-vX.Y.Z-macos-universal`: macOS on Apple Silicon and Intel
- `viddeck-vX.Y.Z-x86_64-windows.exe`: Windows

### From source

With the [Rust toolchain](https://rustup.rs/) installed:

```bash
cargo build --release
```

The program is then `target/release/viddeck`.

### FFmpeg

VidDeck needs `ffmpeg` and `ffprobe`, the programs of FFmpeg. Homebrew installs
them with VidDeck; otherwise install FFmpeg with your package manager:

- **Ubuntu/Debian**: `sudo apt install ffmpeg`
- **Arch Linux**: `sudo pacman -S ffmpeg`
- **macOS**: `brew install ffmpeg`

VidDeck looks for both in `PATH`. To use another FFmpeg installation, pass
`--ffmpeg /path/to/ffmpeg` or set `VIDDECK_FFMPEG`; `ffprobe` is then taken from
the same directory. VidDeck stops at startup if it cannot run them, and warns if
FFmpeg is older than 4.3.

## Usage

```bash
viddeck [OPTIONS] [PATH]
```

`PATH` is the directory with the videos, by default the current directory.
VidDeck prints the address of its web interface, `http://127.0.0.1:8765` unless
you change it; open that address in a browser. Ctrl+C stops VidDeck.

At every start, VidDeck reads all videos with `ffprobe`. For a large collection
this takes a while. The browser shows the library as soon as the first videos
are found, and the complete list when the scan is done.

| Option | |
|---|---|
| `--host HOST` | Address to listen on, by default `127.0.0.1`: an IP address (IPv6 with or without brackets) or a hostname |
| `--port PORT` | Port to listen on, by default `8765` |
| `-w`, `--watch` | Follow changes to the directory while VidDeck runs |
| `--remote` | Replace the System, Browser and Folder buttons with one Play button for playback in the browser |
| `--read-only` | Disable renaming |
| `--ffmpeg PATH` | The `ffmpeg` program to use, see [FFmpeg](#ffmpeg); also `VIDDECK_FFMPEG` |

```bash
# The videos in the current directory
viddeck

# Another directory, on another port
viddeck --port 8080 /path/to/videos

# Follow changes to the directory
viddeck --watch /path/to/videos

# Share on the local network, without the system buttons and without renaming
viddeck --host 0.0.0.0 --remote --read-only /path/to/videos
```

### Sharing on a network

VidDeck has no authentication. By default it listens on `127.0.0.1`, so only
the machine it runs on can reach it. With another address, anyone who can reach
the port can browse and play the whole library, and rename files unless
`--read-only` is set. Share it only on networks you trust.

The System and Folder buttons open a program on the machine VidDeck runs on, so
they work only in a browser on that machine. `--remote` replaces them with
playback in the browser, for everyone else.

## License

Apache-2.0. See [LICENSE](LICENSE). Releases up to 0.4.1 were published under the
BSD 3-Clause License.
