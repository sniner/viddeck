use axum::{
    extract::{Path, Query, State, ConnectInfo},
    response::{Html, IntoResponse, Response, Json, sse::{Event, Sse}},
    http::{StatusCode, header},
    Form,
};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, LazyLock};
use std::collections::HashMap;
use futures_util::stream::Stream;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use serde::{Deserialize, Serialize};

use tower_http::services::ServeFile;
use tower::ServiceExt;

use tokio::sync::Semaphore;
use tokio_util::io::ReaderStream;
use axum::body::Body;

use crate::state::AppState;
use crate::assets::{STYLESHEET, JAVASCRIPT};
use crate::html::generate_shell_html;
use crate::ffmpeg::{render_thumb, transcode_video};
use base64::Engine;

// --- Index (HTML shell) ---

pub async fn index_handler() -> impl IntoResponse {
    Html(generate_shell_html())
}

// --- Static assets ---

pub async fn style_handler() -> impl IntoResponse {
    ([
        (header::CONTENT_TYPE, "text/css"),
        (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
    ], STYLESHEET)
}

pub async fn script_handler() -> impl IntoResponse {
    ([
        (header::CONTENT_TYPE, "application/javascript"),
        (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
    ], JAVASCRIPT)
}

// --- /api/videos ---

#[derive(Serialize)]
struct ApiVideosResponse {
    root: String,
    scanning: bool,
    remote: bool,
    videos: HashMap<String, ApiVideoEntry>,
}

#[derive(Serialize)]
struct ApiVideoEntry {
    rel_path: String,
    duration: f64,
    size: u64,
    width: u32,
    height: u32,
    fps: f64,
    codec: String,
    audio_codecs: Vec<String>,
    chapters: Vec<ApiChapter>,
}

#[derive(Serialize)]
struct ApiChapter {
    start: f64,
    end: f64,
    title: String,
}

pub async fn api_videos_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let scanning = *state.scanning.read();
    let root = state.root.to_string_lossy().to_string();

    let videos = state.videos.read();
    let mut api_videos = HashMap::with_capacity(videos.len());

    for (id, entry) in videos.iter() {
        let chapters = entry.meta.chapters.iter().map(|ch| ApiChapter {
            start: ch.start,
            end: ch.end,
            title: ch.title.clone(),
        }).collect();

        api_videos.insert(id.clone(), ApiVideoEntry {
            rel_path: entry.rel_path.to_string_lossy().to_string(),
            duration: entry.meta.duration,
            size: entry.meta.size,
            width: entry.meta.width,
            height: entry.meta.height,
            fps: entry.meta.fps,
            codec: entry.meta.codec.clone(),
            audio_codecs: entry.meta.audio_codecs.clone(),
            chapters,
        });
    }

    Json(ApiVideosResponse {
        root,
        scanning,
        remote: state.remote,
        videos: api_videos,
    })
}

// --- Thumbnails ---

#[derive(Deserialize)]
pub struct ThumbParams {
    pub mode: Option<String>,
    pub offset: Option<f64>,
    pub width: Option<u32>,
}

use moka::future::Cache;

static THUMB_CACHE: LazyLock<Cache<String, Vec<u8>>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(512)
        .build()
});

pub async fn thumb_handler(
    State(state): State<Arc<AppState>>,
    Path((id, idx_str)): Path<(String, String)>,
    Query(params): Query<ThumbParams>,
) ->  Result<impl IntoResponse, StatusCode> {
    let mode = params.mode.unwrap_or_else(|| "percent".to_string());
    let offset = params.offset.unwrap_or(50.0);
    let mut width = params.width.unwrap_or(1280);

    // Parse idx from "0.jpg" -> 0
    let idx = idx_str.trim_end_matches(".jpg").parse::<usize>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Clamp width
    if width > 1920 { width = 1920; }
    if width < 100 && width != 0 { width = 640; }

    let cache_key = format!("{id}-{idx}-{mode}-{offset}-{width}");

    if let Some(data) = THUMB_CACHE.get(&cache_key).await {
        return Ok(([(header::CONTENT_TYPE, "image/jpeg")], data).into_response());
    }

    // Lookup path and chapter from videos HashMap
    let (path, chapter) = {
        let videos = state.videos.read();
        let entry = videos.get(&id).ok_or(StatusCode::NOT_FOUND)?;
        let ch = entry.meta.chapters.get(idx).cloned().ok_or(StatusCode::NOT_FOUND)?;
        (entry.path.clone(), ch)
    };

    // Calculate timestamp
    let length = (chapter.end - chapter.start).max(0.01);
    let ts = if mode == "seconds" {
        chapter.start + offset
    } else {
        let p = offset.clamp(0.0, 100.0) / 100.0;
        chapter.start + (p * length)
    };
    let ts = ts.clamp(chapter.start, chapter.end);

    // Render
    #[allow(clippy::cast_possible_truncation)] // width is clamped to <= 1920
    let width_u16 = width as u16;
    match render_thumb(&path, ts, width_u16).await {
        Ok(data) => {
            THUMB_CACHE.insert(cache_key, data.clone()).await;
            Ok(([(header::CONTENT_TYPE, "image/jpeg")], data).into_response())
        }
        Err(e) => {
            eprintln!("[thumb] Failed to render thumbnail for {}: {e}", path.display());
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// --- Video streaming ---

pub async fn video_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    req: axum::extract::Request,
) -> Result<Response, StatusCode> {
    let path = {
        let videos = state.videos.read();
        videos.get(&id).map(|e| e.path.clone())
    };

    let path = path.ok_or(StatusCode::NOT_FOUND)?;

    // ServeFile handles Range headers automatically
    let service = ServeFile::new(path);
    let result = service.oneshot(req).await;

    match result {
        Ok(response) => Ok(response.into_response()),
        Err(e) => {
            eprintln!("[video] Failed to serve file: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// --- Video transcoding ---

#[derive(Deserialize)]
pub struct TranscodeParams {
    pub t: Option<f64>,
}

static TRANSCODE_SEM: LazyLock<Semaphore> = LazyLock::new(|| Semaphore::new(2));

const BROWSER_COMPAT_VIDEO: &[&str] = &["h264", "vp8", "vp9", "av1"];

pub async fn transcode_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<TranscodeParams>,
) -> Result<Response, StatusCode> {
    let (path, video_codec) = {
        let videos = state.videos.read();
        let entry = videos.get(&id).ok_or(StatusCode::NOT_FOUND)?;
        (entry.path.clone(), entry.meta.codec.clone())
    };

    let _permit = TRANSCODE_SEM.acquire().await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let transcode_video_stream = !BROWSER_COMPAT_VIDEO.iter()
        .any(|ok| video_codec.eq_ignore_ascii_case(ok));

    let start_time = params.t.unwrap_or(0.0);
    let mut child = transcode_video(&path, start_time, transcode_video_stream).await
        .map_err(|e| {
            eprintln!("[transcode] Failed to start ffmpeg: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let stdout = child.stdout.take().ok_or_else(|| {
        eprintln!("[transcode] No stdout from ffmpeg");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Spawn a task to wait for the child and log errors
    tokio::spawn(async move {
        match child.wait().await {
            Ok(status) if !status.success() => {
                if let Some(mut stderr) = child.stderr.take() {
                    use tokio::io::AsyncReadExt;
                    let mut buf = Vec::new();
                    let _ = stderr.read_to_end(&mut buf).await;
                    let msg = String::from_utf8_lossy(&buf);
                    if !msg.is_empty() {
                        eprintln!("[transcode] ffmpeg stderr: {msg}");
                    }
                }
            }
            Err(e) => eprintln!("[transcode] Failed to wait for ffmpeg: {e}"),
            _ => {}
        }
    });

    let stream = ReaderStream::new(stdout);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "video/mp4")
        .body(body)
        .unwrap()
        .into_response())
}

// --- API: open file / open directory ---

#[derive(Deserialize)]
pub struct ApiOpenParams {
    pub id: String,
}

fn is_localhost(addr: &SocketAddr) -> bool {
    match addr.ip() {
        std::net::IpAddr::V4(ip) => ip.is_loopback(),
        std::net::IpAddr::V6(ip) => ip.is_loopback(),
    }
}

pub async fn api_open_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<ApiOpenParams>,
) -> Result<&'static str, StatusCode> {
    open_path(&state, &params.id, false, &addr)
}

pub async fn api_open_dir_handler(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<ApiOpenParams>,
) -> Result<&'static str, StatusCode> {
    open_path(&state, &params.id, true, &addr)
}

fn open_path(state: &AppState, id: &str, dir: bool, addr: &SocketAddr) -> Result<&'static str, StatusCode> {
    if state.remote || !is_localhost(addr) {
        return Err(StatusCode::FORBIDDEN);
    }

    let path = {
        let videos = state.videos.read();
        videos.get(id).map(|e| e.path.clone())
    };

    if let Some(path) = path {
        let target = if dir {
            path.parent().unwrap_or(&path).to_path_buf()
        } else {
            path
        };

        match open::that(target) {
            Ok(()) => Ok("OK"),
            Err(e) => {
                eprintln!("[open] Failed to open path: {e}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// --- API: rename ---

#[derive(Serialize)]
pub struct RenameResponse {
    old_id: String,
    new_id: String,
    rel_path: String,
}

pub async fn api_rename_handler(
    State(state): State<Arc<AppState>>,
    Form(params): Form<HashMap<String, String>>,
) -> Result<Json<RenameResponse>, (StatusCode, String)> {
    let id = params.get("id").ok_or((StatusCode::BAD_REQUEST, "Missing id".into()))?;
    let new_name = params.get("new_name").ok_or((StatusCode::BAD_REQUEST, "Missing new_name".into()))?;

    // Get old path
    let old_path = {
        let videos = state.videos.read();
        videos.get(id).map(|e| e.path.clone())
    };

    let old_path = old_path.ok_or((StatusCode::NOT_FOUND, "File not found".into()))?;

    // Validate new_name: reject obvious traversal attempts
    if new_name.contains("..") || new_name.starts_with('/') || new_name.starts_with('\\') {
        return Err((StatusCode::BAD_REQUEST, "Invalid filename".into()));
    }

    let new_path = state.root.join(new_name);

    // Canonicalize parent to resolve symlinks and verify the path stays within root
    let parent = new_path.parent()
        .ok_or((StatusCode::BAD_REQUEST, "Invalid path".into()))?;

    // Create parent dirs if needed (before canonicalize)
    tokio::fs::create_dir_all(parent).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let canonical_parent = parent.canonicalize()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let canonical_root = state.root.canonicalize()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !canonical_parent.starts_with(&canonical_root) {
        return Err((StatusCode::BAD_REQUEST, "Path escapes root directory".into()));
    }

    if new_path.exists() {
         return Err((StatusCode::CONFLICT, "File already exists".into()));
    }

    tokio::fs::rename(&old_path, &new_path).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_id = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(new_path.to_string_lossy().as_bytes());
    let new_rel_path = new_path.strip_prefix(&state.root).unwrap_or(&new_path).to_path_buf();
    let rel_path_str = new_rel_path.to_string_lossy().to_string();

    {
        let mut videos = state.videos.write();
        if let Some(mut entry) = videos.remove(id) {
            entry.id.clone_from(&new_id);
            entry.path = new_path;
            entry.rel_path = new_rel_path;
            videos.insert(new_id.clone(), entry);
        }
    }

    Ok(Json(RenameResponse {
        old_id: id.clone(),
        new_id,
        rel_path: rel_path_str,
    }))
}

// --- SSE ---

pub async fn sse_handler(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|res: Result<(), tokio_stream::wrappers::errors::BroadcastStreamRecvError>| {
        if res.is_ok() {
            Some(Ok(Event::default().data("refresh")))
        } else {
            None
        }
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}
