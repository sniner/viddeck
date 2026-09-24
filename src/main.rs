mod assets;
mod cli;
mod ffmpeg;
mod handlers;
mod html;
mod scanner;
mod state;

use axum::{
    Router,
    routing::{get, post},
};
use clap::Parser;
use cli::Args;
use handlers::{
    api_open_dir_handler, api_open_handler, api_rename_handler, api_videos_handler, index_handler,
    logo_handler, script_handler, sse_handler, style_handler, thumb_handler, transcode_handler,
    video_handler,
};
use scanner::scan_library;
use state::AppState;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse arguments
    let args = Args::parse();

    // Resolve absolute path
    let root = tokio::fs::canonicalize(&args.path)
        .await
        .map_err(|_| anyhow::anyhow!("{} does not exist.", args.path.display()))?;

    // Configure and verify the ffmpeg tools before anything uses them
    ffmpeg::configure_tools(args.ffmpeg.clone());
    ffmpeg::check_tools().await?;

    let addr = listen_addr(&args.host, args.port).await?;

    println!("Scanning {} for videos...", root.display());

    if !addr.ip().is_loopback() {
        eprintln!(
            "WARNING: {} can be reached from other machines, and VidDeck has no authentication.",
            args.host
        );
        if !args.read_only {
            eprintln!(
                "WARNING: anyone who can reach it can rename files; --read-only prevents that."
            );
        }
        if !args.remote {
            eprintln!(
                "Note: the System and Folder buttons work only in a browser on this machine; \
                 --remote replaces them with playback in the browser."
            );
        }
    }

    if args.remote {
        println!("Remote mode: videos play in the browser, the System and Folder buttons are off.");
    }

    // Init State
    // Initial receiver is unused; subscribers are created via tx.subscribe()
    let (tx, _rx) = tokio::sync::broadcast::channel(100);
    let state = Arc::new(AppState::new(root.clone(), args.remote, args.read_only, tx));

    // Start background scan
    let state_clone = state.clone();
    tokio::spawn(async move {
        scan_library(state_clone).await;
    });

    if args.watch {
        crate::scanner::start_watcher(state.clone());
    }

    // Setup Router
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/style.css", get(style_handler))
        .route("/script.js", get(script_handler))
        .route("/viddeck.svg", get(logo_handler))
        .route("/thumb/:id/:idx", get(thumb_handler))
        .route("/video/:id", get(video_handler))
        .route("/video/:id/transcode", get(transcode_handler))
        .route("/api/videos", get(api_videos_handler))
        .route("/api/open_file", post(api_open_handler))
        .route("/api/open_dir", post(api_open_dir_handler))
        .route("/api/rename", post(api_rename_handler))
        .route("/api/events", get(sse_handler))
        .with_state(state);

    println!("\nVidDeck is running at http://{addr}");
    println!("Press Ctrl+C to stop.");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

/// The address to listen on for `--host` and `--port`. The host is an IP
/// address, an IPv6 address in brackets as in a URL, or a hostname.
async fn listen_addr(host: &str, port: u16) -> anyhow::Result<SocketAddr> {
    if let Some(ip) = parse_ip(host) {
        return Ok(SocketAddr::new(ip, port));
    }
    tokio::net::lookup_host((host, port))
        .await
        .ok()
        .and_then(|mut addrs| addrs.next())
        .ok_or_else(|| anyhow::anyhow!("cannot resolve host {host}"))
}

fn parse_ip(host: &str) -> Option<IpAddr> {
    let bare = host
        .strip_prefix('[')
        .and_then(|h| h.strip_suffix(']'))
        .unwrap_or(host);
    bare.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn ip_addresses_parse() {
        assert_eq!(parse_ip("127.0.0.1"), Some(IpAddr::V4(Ipv4Addr::LOCALHOST)));
        assert_eq!(parse_ip("::1"), Some(IpAddr::V6(Ipv6Addr::LOCALHOST)));
    }

    #[test]
    fn bracketed_ipv6_parses() {
        assert_eq!(parse_ip("[::1]"), Some(IpAddr::V6(Ipv6Addr::LOCALHOST)));
    }

    #[test]
    fn hostnames_and_broken_brackets_are_no_ip() {
        assert_eq!(parse_ip("localhost"), None);
        assert_eq!(parse_ip("[localhost]"), None);
        assert_eq!(parse_ip("[::1"), None);
        assert_eq!(parse_ip("[127.0.0.1]:80"), None);
    }
}
