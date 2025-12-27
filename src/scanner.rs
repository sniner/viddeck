use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;


use crate::state::{AppState, VideoEntry};
use crate::ffmpeg::get_extended_metadata;

const VIDEO_EXTENSIONS: &[&str] = &["mkv", "mp4", "m4v", "avi", "mov", "webm"];

pub async fn scan_library(state: Arc<AppState>) {
    let root = state.root.clone();
    
    // We run the blocking walkdir in a blocking task to not block the async runtime
    tokio::task::spawn_blocking(move || {
        let walker = WalkDir::new(&root).into_iter();
        
        // Collect valid files first (cheap)
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

        // Now process them (expensive, involves ffprobe)
        // We can do this in parallel using rayon if we wanted, 
        // but for now we iterate and spawn async tasks or just do it here.
        // Since get_extended_metadata is async, we need a runtime handle.
        
        let rt = tokio::runtime::Handle::current();
        
        for path in candidates {
            let state_clone = state.clone();
            let root_clone = root.clone();
            
            // We block on the async metadata call here for simplicity in this thread, 
            // or we could dispatch.
            rt.block_on(async move {
               match get_extended_metadata(&path).await {
                   Ok(meta) => {
                       if meta.duration > 0.0 {
                           let rel_path = path.strip_prefix(&root_clone).unwrap_or(&path).to_path_buf();
                           let id = URL_SAFE_NO_PAD.encode(path.to_string_lossy().as_bytes());
                           
                           let entry = VideoEntry {
                               id,
                               path,
                               rel_path,
                               meta,
                           };
                           state_clone.add_entry(entry);
                       }
                   },
                   Err(_) => {} // skip failed
               } 
            });
        }
        
        state.sort_entries();
        *state.scanning.write().unwrap() = false;
        
    }).await.unwrap();
}
