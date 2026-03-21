use std::sync::Arc;
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use walkdir::WalkDir;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

use crate::state::{AppState, VideoEntry};
use crate::ffmpeg::get_extended_metadata;

const VIDEO_EXTENSIONS: &[&str] = &["mkv", "mp4", "m4v", "avi", "mov", "webm"];

/// Debounce window: collect filesystem events for this long before processing.
const WATCHER_DEBOUNCE_MS: u64 = 2000;

/// Max parallel ffprobe calls during scan or watcher batch processing.
const MAX_CONCURRENT_SCANS: usize = 4;

pub fn is_video_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| VIDEO_EXTENSIONS.contains(&e.to_lowercase().as_str()))
}

pub async fn process_single_video(path: &Path, root: &Path, state: &Arc<AppState>) -> bool {
    let id = URL_SAFE_NO_PAD.encode(path.to_string_lossy().as_bytes());
    let current_modified = std::fs::metadata(path).ok().and_then(|m| m.modified().ok());

    {
        let videos = state.videos.read();
        if let Some(existing) = videos.iter().find(|v| v.id == id)
            && existing.modified == current_modified
            && current_modified.is_some()
        {
            return false;
        }
    }

    if !is_video_file(path) {
        return false;
    }

    if let Ok(meta) = get_extended_metadata(path).await
        && meta.duration > 0.0
    {
        let rel_path = path.strip_prefix(root).unwrap_or(path).to_path_buf();

        {
            let mut id_map = state.id_map.write();
            if id_map.remove(&id).is_some() {
                let mut videos = state.videos.write();
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
    false
}

pub async fn scan_library(state: Arc<AppState>) {
    let root = state.root.clone();

    let candidates = tokio::task::spawn_blocking({
        let root = root.clone();
        move || {
            WalkDir::new(&root)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.path().is_file() && is_video_file(e.path()))
                .map(|e| e.path().to_path_buf())
                .collect::<Vec<_>>()
        }
    })
    .await
    .unwrap();

    let sem = Arc::new(tokio::sync::Semaphore::new(MAX_CONCURRENT_SCANS));
    let mut join_set = tokio::task::JoinSet::new();

    for path in candidates {
        let state = state.clone();
        let root = root.clone();
        let sem = sem.clone();
        join_set.spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            process_single_video(&path, &root, &state).await
        });
    }

    while join_set.join_next().await.is_some() {}

    state.sort_entries();
    *state.scanning.write() = false;
    state.notify_refresh();
}

pub fn start_watcher(state: Arc<AppState>) {
    tokio::spawn(async move {
        if let Err(e) = run_watcher(state).await {
            eprintln!("[watcher] Watcher stopped with error: {e}");
        }
    });
}

#[allow(clippy::too_many_lines)]
async fn run_watcher(state: Arc<AppState>) -> anyhow::Result<()> {
    use notify::{Watcher, RecursiveMode, EventKind};
    use std::time::Duration;

    let root = state.root.clone();
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let root_clone = root.clone();
    let watcher = tokio::task::spawn_blocking(move || {
        let mut watcher = match notify::recommended_watcher(
            move |res: notify::Result<notify::Event>| {
                if let Ok(event) = res {
                    if matches!(event.kind, EventKind::Access(_)) {
                        return;
                    }
                    for path in event.paths {
                        let _ = tx.blocking_send(path);
                    }
                }
            },
        ) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("[watcher] Failed to create watcher: {e}");
                return None;
            }
        };

        if let Err(e) = watcher.watch(&root_clone, RecursiveMode::Recursive) {
            eprintln!("[watcher] Failed to watch {}: {e}", root_clone.display());
            return None;
        }

        Some(watcher)
    })
    .await
    .unwrap();

    if watcher.is_none() {
        anyhow::bail!("File watching disabled due to setup error.");
    }
    let _watcher = watcher; // keep alive

    println!("[watcher] Watching {} for changes.", root.display());

    loop {
        let Some(first_path) = rx.recv().await else { break };

        let mut pending: HashSet<PathBuf> = HashSet::new();
        pending.insert(first_path);

        let deadline = tokio::time::sleep(Duration::from_millis(WATCHER_DEBOUNCE_MS));
        tokio::pin!(deadline);

        loop {
            tokio::select! {
                () = &mut deadline => break,
                res = rx.recv() => match res {
                    Some(p) => { pending.insert(p); }
                    None => return Ok(()),
                }
            }
        }

        // Phase 1: handle deletions
        let mut to_remove: Vec<String> = Vec::new();
        for path in &pending {
            if !path.exists() {
                let id_map = state.id_map.read();
                for (vid_id, vid_path) in id_map.iter() {
                    if vid_path.starts_with(path) {
                        to_remove.push(vid_id.clone());
                    }
                }
            }
        }
        let had_deletions = !to_remove.is_empty();
        if had_deletions {
            let mut id_map = state.id_map.write();
            let mut videos = state.videos.write();
            for rid in &to_remove {
                id_map.remove(rid);
                videos.retain(|v| &v.id != rid);
            }
        }

        // Phase 2: collect unique file paths to process
        let mut to_process: HashSet<PathBuf> = HashSet::new();
        for path in &pending {
            if !path.exists() {
                continue;
            }
            if path.is_dir() {
                for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
                    let p = entry.path().to_path_buf();
                    if p.is_file() && is_video_file(&p) {
                        to_process.insert(p);
                    }
                }
            } else if path.is_file() && is_video_file(path) {
                to_process.insert(path.clone());
            }
        }

        // Phase 3: process in parallel
        let sem = Arc::new(tokio::sync::Semaphore::new(MAX_CONCURRENT_SCANS));
        let mut join_set = tokio::task::JoinSet::new();
        let root = state.root.clone();

        for path in to_process {
            let state = state.clone();
            let root = root.clone();
            let sem = sem.clone();
            join_set.spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                process_single_video(&path, &root, &state).await
            });
        }

        let mut any_changed = had_deletions;
        while let Some(result) = join_set.join_next().await {
            if result.unwrap_or(false) {
                any_changed = true;
            }
        }

        if any_changed {
            state.sort_entries();
            state.notify_refresh();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_extensions_recognized() {
        for ext in VIDEO_EXTENSIONS {
            let path = PathBuf::from(format!("test.{ext}"));
            assert!(is_video_file(&path), "should recognize .{ext}");
        }
    }

    #[test]
    fn video_extensions_case_insensitive() {
        assert!(is_video_file(Path::new("file.MKV")));
        assert!(is_video_file(Path::new("file.Mp4")));
    }

    #[test]
    fn non_video_rejected() {
        assert!(!is_video_file(Path::new("file.txt")));
        assert!(!is_video_file(Path::new("file.jpg")));
        assert!(!is_video_file(Path::new("file")));
    }
}
