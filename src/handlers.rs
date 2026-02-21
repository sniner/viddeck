use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse, Response},
    http::{StatusCode, header},
    Form,
};
use std::sync::Arc;
use serde::Deserialize;

use tower_http::services::ServeFile;
use tower::ServiceExt; 

use crate::state::AppState;
use crate::assets::{STYLESHEET, JAVASCRIPT};
use crate::html::generate_index_html;
use crate::ffmpeg::render_thumb;
use base64::Engine;

// Query Params
#[derive(Deserialize)]
pub struct IndexParams {
    pub mode: Option<String>,
    pub value: Option<f64>,
    pub width: Option<u32>,
}

pub async fn index_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<IndexParams>,
) -> impl IntoResponse {
    let mode = params.mode.unwrap_or_else(|| "percent".to_string());
    let value = params.value.unwrap_or(50.0);
    let width = params.width.unwrap_or(1280);
    
    // Check scanning status
    let scanning = *state.scanning.read().unwrap();
    let count = state.videos.read().unwrap().len();
    let root = state.root.to_string_lossy().to_string();

    // Clone videos for rendering to avoid holding lock too long
    // (In a real app with thousands of videos, we might want pagination)
    let videos = {
        let v = state.videos.read().unwrap();
        v.clone()
    };

    let html = generate_index_html(&videos, count, &root, &mode, value, width, scanning);
    Html(html)
}

pub async fn style_handler() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], STYLESHEET)
}

pub async fn script_handler() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/javascript")], JAVASCRIPT)
}

#[derive(Deserialize)]
pub struct ThumbParams {
    pub mode: Option<String>,
    pub value: Option<f64>,
    pub width: Option<u32>,
}

// Simple in-memory LRU cache using moka
// We declare it in main or use a global static for simplicity in this script-like tool
use lazy_static::lazy_static;
use moka::future::Cache;

lazy_static! {
    static ref THUMB_CACHE: Cache<String, Vec<u8>> = Cache::builder()
        .max_capacity(512)
        .build();
}

pub async fn thumb_handler(
    State(state): State<Arc<AppState>>,
    Path((id, idx_str)): Path<(String, String)>,
    Query(params): Query<ThumbParams>,
) ->  Result<impl IntoResponse, StatusCode> {
    let mode = params.mode.unwrap_or_else(|| "percent".to_string());
    let value = params.value.unwrap_or(50.0);
    let mut width = params.width.unwrap_or(1280);
    
    // Parse idx from "0.jpg" -> 0
    let idx = idx_str.trim_end_matches(".jpg").parse::<usize>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Clamp width
    if width > 1920 { width = 1920; }
    if width < 100 && width != 0 { width = 640; }

    let cache_key = format!("{}-{}-{}-{}-{}", id, idx, mode, value, width);

    if let Some(data) = THUMB_CACHE.get(&cache_key).await {
        return Ok(([(header::CONTENT_TYPE, "image/jpeg")], data).into_response());
    }

    // Lookup path
    let path = {
        let map = state.id_map.read().unwrap();
        map.get(&id).cloned()
    };
    
    let path = path.ok_or(StatusCode::NOT_FOUND)?;
    
    // Find chapter info
    let chapter = {
        let videos = state.videos.read().unwrap();
        let entry = videos.iter().find(|v| v.id == id).ok_or(StatusCode::NOT_FOUND)?;
        entry.meta.chapters.get(idx).cloned()
    };
    
    let chapter = chapter.ok_or(StatusCode::NOT_FOUND)?;

    // Calculate timestamp
    let length = (chapter.end - chapter.start).max(0.01);
    let ts = if mode == "seconds" {
        chapter.start + value
    } else {
        let p = value.clamp(0.0, 100.0) / 100.0;
        chapter.start + (p * length)
    };
    let ts = ts.clamp(chapter.start, chapter.end);

    // Render
    match render_thumb(&path, ts, width as u16).await {
        Ok(data) => {
            THUMB_CACHE.insert(cache_key, data.clone()).await;
            Ok(([(header::CONTENT_TYPE, "image/jpeg")], data).into_response())
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn video_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    req: axum::extract::Request,
) -> Result<Response, StatusCode> {
    let path = {
        let map = state.id_map.read().unwrap();
        map.get(&id).cloned()
    };
    
    let path = path.ok_or(StatusCode::NOT_FOUND)?;
    
    // ServeFile handles Range headers automatically
    let service = ServeFile::new(path);
    let result = service.oneshot(req).await;
    
    match result {
        Ok(response) => Ok(response.into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// API Handlers

#[derive(Deserialize)]
pub struct ApiOpenParams {
    pub id: String,
}

pub async fn api_open_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ApiOpenParams>,
) -> Result<&'static str, StatusCode> {
    open_path(&state, &params.id, false)
}

pub async fn api_open_dir_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ApiOpenParams>,
) -> Result<&'static str, StatusCode> {
    open_path(&state, &params.id, true)
}

fn open_path(state: &AppState, id: &str, dir: bool) -> Result<&'static str, StatusCode> {
    let path = {
        let map = state.id_map.read().unwrap();
        map.get(id).cloned()
    };
    
    if let Some(path) = path {
        let target = if dir {
            path.parent().unwrap_or(&path).to_path_buf()
        } else {
            path
        };
        
        // Use 'open' crate
        match open::that(target) {
            Ok(_) => Ok("OK"),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn api_rename_handler(
    State(state): State<Arc<AppState>>,
    Form(params): Form<std::collections::HashMap<String, String>>,
) -> Result<&'static str, (StatusCode, String)> {
    let id = params.get("id").ok_or((StatusCode::BAD_REQUEST, "Missing id".into()))?;
    let new_name = params.get("new_name").ok_or((StatusCode::BAD_REQUEST, "Missing new_name".into()))?;
    
    // Logic to rename
    // 1. Get old path
    let old_path = {
        let map = state.id_map.read().unwrap();
        map.get(id).cloned()
    };
    
    let old_path = old_path.ok_or((StatusCode::NOT_FOUND, "File not found".into()))?;
    
    // 2. Resolve new path (relative to root)
    // Security check: simple check against '..' traversal
    if new_name.contains("..") || new_name.starts_with("/") || new_name.starts_with("\\") {
        return Err((StatusCode::BAD_REQUEST, "Invalid filename".into()));
    }

    let new_path = state.root.join(new_name); // This assumes new_name is relative path?
    // Wait, the python scripts says "Treat new_name as relative to root"
    // Let's stick to that logic
    
    if new_path.exists() {
         return Err((StatusCode::CONFLICT, "File already exists".into()));
    }
    
    // Create parent dirs if needed
    if let Some(parent) = new_path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }
    
    // Rename
    tokio::fs::rename(&old_path, &new_path).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Update State
    // We need to update ID map and VideoEntry list
    // Since we hold a write lock, we should be careful.
    // For simplicity, we just trigger a rescan or update in place.
    // Updating in place is faster.
    
    let new_id = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(new_path.to_string_lossy().as_bytes());
    let new_rel_path = new_path.strip_prefix(&state.root).unwrap_or(&new_path).to_path_buf();

    {
        let mut videos = state.videos.write().unwrap();
        let mut map = state.id_map.write().unwrap();
        
        // Remove old map entry
        map.remove(id);
        // Add new map entry
        map.insert(new_id.clone(), new_path.clone());
        
        // Find and update video entry
        if let Some(entry) = videos.iter_mut().find(|v| v.id == *id) {
            entry.id = new_id;
            entry.path = new_path;
            entry.rel_path = new_rel_path;
        }
    }

    Ok("OK")
}
