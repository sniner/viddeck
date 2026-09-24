use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory with the videos
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Port to listen on
    #[arg(long, default_value_t = 8765)]
    pub port: u16,

    /// Address to listen on: an IP address (IPv6 with or without brackets) or a hostname
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Follow changes to the directory while running
    #[arg(short, long)]
    pub watch: bool,

    /// Replace the System, Browser and Folder buttons with playback in the browser
    #[arg(long)]
    pub remote: bool,

    /// Disable renaming
    #[arg(long)]
    pub read_only: bool,

    /// The ffmpeg program to use; ffprobe is taken from the same directory
    #[arg(long, env = "VIDDECK_FFMPEG")]
    pub ffmpeg: Option<PathBuf>,
}
