use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Root directory to scan
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Port to bind
    #[arg(long, default_value_t = 8765)]
    pub port: u16,

    /// Host to bind
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Watch for filesystem changes
    #[arg(short, long)]
    pub watch: bool,

    /// Remote mode: disable system open commands (no local player/folder buttons)
    #[arg(long)]
    pub remote: bool,
}
