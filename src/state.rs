use std::path::PathBuf;
use std::sync::RwLock;
use serde::Serialize;
use crate::ffmpeg::VideoMetadata;
use std::collections::HashMap;

#[derive(Clone, Serialize, Debug)]
pub struct VideoEntry {
    pub id: String, // Base64 encoded path
    pub path: PathBuf,
    pub rel_path: PathBuf,
    pub meta: VideoMetadata,
}

pub struct AppState {
    pub root: PathBuf,
    pub videos: RwLock<Vec<VideoEntry>>,
    pub scanning: RwLock<bool>,
    // Map ID -> Path for quick lookup
    pub id_map: RwLock<HashMap<String, PathBuf>>,
    // LRU Cache for thumbnails could stem from a separate Crate, or we handle it in Handler
}

impl AppState {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            videos: RwLock::new(Vec::new()),
            scanning: RwLock::new(true),
            id_map: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_entry(&self, entry: VideoEntry) {
        let mut videos = self.videos.write().unwrap();
        let mut map = self.id_map.write().unwrap();
        
        map.insert(entry.id.clone(), entry.path.clone());
        videos.push(entry);
    }

    pub fn sort_entries(&self) {
        let mut videos = self.videos.write().unwrap();
        videos.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    }
}
