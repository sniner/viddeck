mod cli;
mod ffmpeg;
mod state;
mod scanner;
mod assets;
mod html;
mod handlers;

use clap::Parser;
use std::sync::Arc;
use std::net::SocketAddr;
use axum::{
    routing::{get, post},
    Router,
};
use cli::Args;
use state::AppState;
use scanner::scan_library;
use handlers::{
    index_handler, style_handler, script_handler, thumb_handler, video_handler, transcode_handler,
    api_videos_handler, api_open_handler, api_open_dir_handler, api_rename_handler, sse_handler
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse arguments
    let args = Args::parse();

    // Resolve absolute path
    let root = tokio::fs::canonicalize(&args.path).await
        .map_err(|_| anyhow::anyhow!("Error: {} does not exist.", args.path.display()))?;

    println!("Scanning {} for videos...", root.display());

    // Warn if host is not localhost
    if args.host != "127.0.0.1" && args.host != "::1" && args.host != "localhost" {
        eprintln!("WARNING: Binding to non-localhost address {}. open/open_dir commands will only work from localhost.", args.host);
    }

    if args.remote {
        println!("Remote mode: system open commands disabled.");
    }

    // Init State
    // Initial receiver is unused; subscribers are created via tx.subscribe()
    let (tx, _rx) = tokio::sync::broadcast::channel(100);
    let state = Arc::new(AppState::new(root.clone(), args.remote, tx));

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
        .route("/thumb/:id/:idx", get(thumb_handler))
        .route("/video/:id", get(video_handler))
        .route("/video/:id/transcode", get(transcode_handler))
        .route("/api/videos", get(api_videos_handler))
        .route("/api/open_file", post(api_open_handler))
        .route("/api/open_dir", post(api_open_dir_handler))
        .route("/api/rename", post(api_rename_handler))
        .route("/api/events", get(sse_handler))
        .with_state(state);

    let addr_str = format!("{}:{}", args.host, args.port);
    let addr: SocketAddr = addr_str.parse()?;

    println!("\nStarted VidDeck at http://{addr}");
    println!("Press Ctrl+C to stop.");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}
