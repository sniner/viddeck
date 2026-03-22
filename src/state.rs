use std::path::PathBuf;
use parking_lot::RwLock;
use serde::Serialize;
use crate::ffmpeg::VideoMetadata;
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Clone, Serialize, Debug)]
pub struct VideoEntry {
    pub id: String, // Base64 encoded path
    pub path: PathBuf,
    pub rel_path: PathBuf,
    pub meta: VideoMetadata,
    pub modified: Option<std::time::SystemTime>,
}

pub struct AppState {
    pub root: PathBuf,
    pub videos: RwLock<HashMap<String, VideoEntry>>,
    pub scanning: RwLock<bool>,
    pub remote: bool,
    pub tx: broadcast::Sender<()>,
}

impl AppState {
    pub fn new(root: PathBuf, remote: bool, tx: broadcast::Sender<()>) -> Self {
        Self {
            root,
            videos: RwLock::new(HashMap::new()),
            scanning: RwLock::new(true),
            remote,
            tx,
        }
    }

    pub fn notify_refresh(&self) {
        let _ = self.tx.send(());
    }
}
