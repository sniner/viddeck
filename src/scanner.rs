use std::sync::Arc;
use walkdir::WalkDir;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

use crate::state::{AppState, VideoEntry};
use crate::ffmpeg::get_extended_metadata;

const VIDEO_EXTENSIONS: &[&str] = &["mkv", "mp4", "m4v", "avi", "mov", "webm"];

pub async fn process_single_video(path: &std::path::Path, root: &std::path::Path, state: &Arc<AppState>) -> bool {
    let id = URL_SAFE_NO_PAD.encode(path.to_string_lossy().as_bytes());
    let current_modified = std::fs::metadata(path).ok().and_then(|m| m.modified().ok());

    {
        let videos = state.videos.read().unwrap();
        if let Some(existing) = videos.iter().find(|v| v.id == id) {
            if existing.modified == current_modified && current_modified.is_some() {
                return false; // Already up to date
            }
        }
    }

    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            if VIDEO_EXTENSIONS.contains(&ext_str.to_lowercase().as_str()) {
                if let Ok(meta) = get_extended_metadata(path).await {
                    if meta.duration > 0.0 {
                        let rel_path = path.strip_prefix(root).unwrap_or(path).to_path_buf();
                        
                        // Remove old entry if it exists to replace it
                        {
                            let mut id_map = state.id_map.write().unwrap();
                            if id_map.remove(&id).is_some() {
                                let mut videos = state.videos.write().unwrap();
                                videos.retain(|v| v.id != id);
                            }
                        }

                        let entry = VideoEntry {
                            id,
                            path: path.to_path_buf(),
                            rel_path,
                            meta,
                            modified: current_modified,
                        };
                        state.add_entry(entry);
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub async fn scan_library(state: Arc<AppState>) {
    let root = state.root.clone();
    
    // We run the walkdir in a blocking task to not block the async runtime
    tokio::task::spawn_blocking(move || {
        let walker = WalkDir::new(&root).into_iter();
        
        let mut candidates = Vec::new();

        for entry in walker.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if VIDEO_EXTENSIONS.contains(&ext_str.to_lowercase().as_str()) {
                            candidates.push(path.to_path_buf());
                        }
                    }
                }
            }
        }

        let rt = tokio::runtime::Handle::current();
        
        for path in candidates {
            let state_clone = state.clone();
            let root_clone = root.clone();
            
            rt.block_on(async move {
                process_single_video(&path, &root_clone, &state_clone).await;
            });
        }
        
        state.sort_entries();
        *state.scanning.write().unwrap() = false;
        state.notify_refresh();
        
    }).await.unwrap();
}

pub fn start_watcher(state: Arc<AppState>) {
    let root = state.root.clone();
    
    tokio::spawn(async move {
        use notify::{Watcher, RecursiveMode, EventKind};
        use std::time::Duration;
        use std::collections::HashSet;
        
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        
        let root_clone = root.clone();
        let _watcher = tokio::task::spawn_blocking(move || {
            let mut watcher = match notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
                if let Ok(event) = res {
                    // Filter out Access events immediately
                    if matches!(event.kind, EventKind::Access(_)) {
                        return;
                    }
                    for path in event.paths {
                        let _ = tx.blocking_send(path);
                    }
                }
            }) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Failed to start watcher: {}", e);
                    return None;
                }
            };
            
            if let Err(e) = watcher.watch(&root_clone, RecursiveMode::Recursive) {
                eprintln!("Watcher error: {}", e);
                return None;
            }
            
            Some(watcher)
        }).await.unwrap();
        
        loop {
            // Wait for the first event
            let first_path = match rx.recv().await {
                Some(p) => p,
                None => break, // Channel closed
            };

            let mut pending = HashSet::new();
            pending.insert(first_path);

            // Wait 2 seconds, collecting any other events that arrive in the meantime
            let timeout = tokio::time::sleep(Duration::from_secs(2));
            tokio::pin!(timeout);

            loop {
                tokio::select! {
                    _ = &mut timeout => {
                        break;
                    }
                    res = rx.recv() => {
                        if let Some(p) = res {
                            pending.insert(p);
                        } else {
                            return; // closed
                        }
                    }
                }
            }

            let mut changed = false;
            
            for path in pending {
                if !path.exists() {
                    let mut id_map = state.id_map.write().unwrap();
                    let mut videos = state.videos.write().unwrap();
                    
                    let mut to_remove = Vec::new();
                    for (vid_id, vid_path) in id_map.iter() {
                        if vid_path.starts_with(&path) {
                            to_remove.push(vid_id.clone());
                        }
                    }
                    
                    for rid in to_remove {
                        id_map.remove(&rid);
                        videos.retain(|v| v.id != rid);
                        changed = true;
                    }
                } else if path.is_dir() {
                    let walker = walkdir::WalkDir::new(&path).into_iter();
                    for entry in walker.filter_map(|e| e.ok()) {
                        let inner_path = entry.path();
                        if inner_path.is_file() {
                            if process_single_video(inner_path, &root, &state).await {
                                changed = true;
                            }
                        }
                    }
                } else if path.is_file() {
                    if process_single_video(&path, &root, &state).await {
                        changed = true;
                    }
                }
            }
            
            if changed {
                state.sort_entries();
                state.notify_refresh();
            }
        }
    });
}
