# Changelog

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- **Homebrew**: `brew install sniner/tap/viddeck` installs VidDeck on macOS, together with FFmpeg
- **Linux on arm64**: releases include a binary for Linux on aarch64

### Changed

- **License**: VidDeck is now licensed under Apache-2.0. Releases up to 0.4.1 remain under the
  BSD 3-Clause License
- **Release binaries** are named after version and platform, e.g.
  `viddeck-v0.5.0-macos-universal`. The Linux binaries are statically linked and run on any
  distribution

## [0.4.1] - 2026-07-10

### Changed

- **Header**: the logo sits on the left and is larger, with title and subtitle beside it

## [0.4.0] - 2026-07-09

### Added

- **`--ffmpeg PATH`**, or the environment variable `VIDDECK_FFMPEG`, selects the FFmpeg
  installation to use; `ffprobe` is taken from the same directory. Without it, both are looked
  up in `PATH` as before
- **Logo** in the page header and as the browser tab icon; it follows the light or dark theme
- **Startup** prints the FFmpeg version and warns if it is older than 4.3

### Changed

- **Startup** stops with an error if `ffmpeg` or `ffprobe` cannot be run. Before, VidDeck
  started and showed no videos

### Fixed

- **Transcoding** failed for every video with an FFmpeg that lists a hardware encoder but lacks
  one of the options VidDeck uses with it (e.g. NVENC presets before FFmpeg 4.3). Such an
  encoder is now skipped

## [0.3.0] - 2026-07-09

### Added

- **`--read-only`** disables renaming: the rename button is hidden and the server refuses
  rename requests
- **Startup warning** when VidDeck listens on an address other than localhost while renaming
  is enabled
- **Keyboard focus** is visible on buttons, selects and inputs
- **README**: a section on sharing VidDeck on a network, which has no authentication, and on
  what `--remote` and `--read-only` do there

### Changed

- **`--host`** takes an IPv6 address without brackets, e.g. `--host ::1`. The bracketed form
  `[::1]` is no longer accepted
- **Transcoding** keeps H.264, VP8, VP9 and AV1 video as it is and converts only the audio,
  the common case being H.264 with AC3 or DTS. This costs no CPU or GPU time and loses no
  quality
- **Search**: while a search is active, the header shows "N of M videos" instead of the size
  of the whole library
- **Chapter durations** on thumbnails have a ⏱️ prefix, so they no longer read as start times
- **Rename field** is wider, so long file names fit
- **Lightbox**: the tooltip of the play button is in English, like the rest of the UI
- **Log**: videos that ffprobe cannot read, or that report no duration, are named in the log
  instead of being skipped silently

### Fixed

- **`--host localhost`** and `--host ::1` failed with "invalid socket address syntax"
- **Transcoding**: the limit of two transcodes at a time was not enforced
- **Transcoding** could hang when FFmpeg wrote a lot of error output
- **Transcoding** failed for every video when FFmpeg listed a hardware encoder that the driver
  or the hardware cannot run. Such an encoder is now skipped, with software encoding
  (libx264) as the last resort
- **Library**: a video for which ffprobe reported no file size was missing from the library
- **Rename**: after renaming a file and then back to its old name, the rename button did
  nothing until the page was reloaded
- **Rename**: a new name like `sub//x.mkv` or `./x.mkv` led to duplicate entries after the
  next update through `--watch`
- **Rename** through a symlink that points outside the library created directories outside
  it, although the rename itself was refused
- **Thumbnails** of a file replaced under the same name stayed outdated
- **Browser playback** declared every file as MP4, which can make a browser refuse an MKV,
  WebM or AVI file
- **System and Folder buttons** no longer hold up the server while the player or file
  manager starts

## [0.2.2] - 2026-03-22

### Changed

- **Transcoding** always converts the video to H.264, also when only the audio needs
  converting
- **Video info**: a "🔄 Transcode" chip marks videos that the browser plays through
  transcoding; the audio chip no longer shows 🔇
- **Log**: "broken pipe" errors from FFmpeg after the player was closed during transcoding
  are no longer printed

## [0.2.1] - 2026-03-22

### Added

- **Transcoding**: browser playback converts a video on the fly when the browser cannot play
  it, that is when the video is not H.264, VP8, VP9 or AV1, or an audio track is not AAC,
  MP3, Opus, Vorbis, FLAC, PCM or MP2. Compatible video is copied; other video is encoded to
  H.264 with a hardware encoder where one is available (VideoToolbox, VAAPI, AMF, NVENC, QSV),
  otherwise with libx264. Audio is converted to AAC

## [0.2.0] - 2026-03-22

### Added

- **`--remote`** is for a browser on another machine: one Play button for browser playback
  replaces the System, Browser and Folder buttons, and the server refuses to open files or
  folders on its own machine
- **Startup warning** when VidDeck listens on an address other than localhost

### Changed

- **Thumbnail settings** (preview position and size) take effect at once; the Update button
  is gone. The settings are no longer part of the URL, so a bookmark no longer carries them
- **Initial scan**: the library appears as soon as the first videos are found, and the
  complete list when the scan is done
- **Renaming** and changes found by `--watch` update the list without reloading the page;
  search and page stay as they were
- **System and Folder buttons** work only from a browser on the machine VidDeck runs on;
  requests from other machines are refused
- **macOS**: the release binary is a universal binary for Apple Silicon and Intel

### Fixed

- **Durations** such as 119.7 seconds were shown as "01:60"

## [0.1.4] - 2026-03-22

### Changed

- **Log**: failures to create a thumbnail, to play a video or to open a file are printed
  instead of being ignored

### Fixed

- **Rename** refuses a new name that leads outside the library through a symlink. Before,
  such a rename could move the file out of the library

## [0.1.3] - 2026-03-20

### Fixed

- **Library**: since 0.1.2, a video was missing from the library when ffprobe reported one of
  its streams without a codec name, as happens with some data streams

## [0.1.2] - 2026-03-20

### Added

- **`--watch`** (`-w`) follows changes to the library: new, changed, moved and deleted files
  show up after about two seconds, and open browser pages reload themselves
- **Chapter playback**: a chapter thumbnail opens in a lightbox with a play button that starts
  browser playback at the chapter
- **Audio chip** lists the audio codecs of a video; 🔇 marks audio that browsers cannot play,
  such as AC3 or DTS
- **Pages**: the library, and a search result, is split into pages of 50 videos

### Changed

- **Look**: shadows, rounder corners, and chapter thumbnails that lift on hover
- **Initial scan** reads up to four files at a time
- **The page** reloads by itself when the initial scan is done
- **Upgrades**: the browser loads the new version of the UI without a forced reload

## [0.1.1] - 2026-02-22

First release. VidDeck needs `ffmpeg` and `ffprobe` in `PATH`.

### Added

- **Library view** of a directory tree: VidDeck scans `PATH` (default: the current directory)
  for MKV, MP4, M4V, AVI, MOV and WebM files and shows one card per video with duration,
  file size, resolution, frame rate and video codec
- **Chapter thumbnails**, one per chapter or one per video without chapters, with title and
  duration; a click enlarges them. The position within the chapter (percent or seconds) and
  the size are adjustable
- **Live search** over the file paths
- **Playback** in the default player of the system or in the browser, and a button that opens
  the containing folder
- **Rename** a file, or move it into a subdirectory of the library
- **Light and dark theme**, following the system setting
- **`--host`** (default `127.0.0.1`) and **`--port`** (default `8765`) set the address
  VidDeck listens on

