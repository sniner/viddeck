mod cli;
mod ffmpeg;
mod state;
mod scanner;
mod assets;
mod html;
mod handlers;

use clap::Parser;
use std::sync::{Arc, RwLock};
use std::net::SocketAddr;
use axum::{
    routing::{get, post},
    Router,
};
use cli::Args;
use state::AppState;
use scanner::scan_library;
use handlers::{
    index_handler, style_handler, script_handler, thumb_handler, video_handler,
    api_open_handler, api_open_dir_handler, api_rename_handler
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse arguments
    let args = Args::parse();
    
    // Resolve absolute path
    let root = tokio::fs::canonicalize(&args.path).await
        .map_err(|_| anyhow::anyhow!("Error: {:?} does not exist.", args.path))?;

    println!("Scanning {:?} for videos...", root);

    // Init State
    let state = Arc::new(AppState::new(root.clone()));
    
    // Start background scan
    let state_clone = state.clone();
    tokio::spawn(async move {
        scan_library(state_clone).await;
        // println!("Scan finished."); 
        // Note: In real app we might want to log this or notify UI via websocket
    });

    // Setup Router
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/style.css", get(style_handler))
        .route("/script.js", get(script_handler))
        .route("/thumb/:id/:idx", get(thumb_handler))
        .route("/video/:id", get(video_handler))
        .route("/api/open_file", post(api_open_handler))
        .route("/api/open_dir", post(api_open_dir_handler))
        .route("/api/rename", post(api_rename_handler))
        .with_state(state);

    let addr_str = format!("{}:{}", args.host, args.port);
    let addr: SocketAddr = addr_str.parse()?;
    
    println!("\nStarted VidDeck at http://{}", addr);
    println!("Loaded videos will appear automatically (refresh page).");
    println!("Press Ctrl+C to stop.");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
