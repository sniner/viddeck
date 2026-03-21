use std::fmt::Write;
use crate::state::VideoEntry;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn human_ts(sec: f64) -> String {
    let total = sec.max(0.0);
    let m = ((total % 3600.0) / 60.0).floor() as u64;
    let s = total % 60.0;
    let h = (total / 3600.0).floor() as u64;
    if h > 0 {
        format!("{h:02}:{m:02}:{s:02.0}")
    } else {
        format!("{m:02}:{s:02.0}")
    }
}

#[allow(clippy::cast_precision_loss)]
pub fn human_size(size_bytes: u64) -> String {
    if size_bytes == 0 {
        return "0 B".to_string();
    }
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size_bytes as f64;
    for unit in units {
        if size < 1024.0 {
            return format!("{size:.1} {unit}");
        }
        size /= 1024.0;
    }
    format!("{size:.1} EB")
}

const PAGE_SIZE: usize = 50;

fn render_chips(meta: &crate::ffmpeg::VideoMetadata) -> String {
    let mut chips = String::new();
    let _ = write!(chips, "<span>⏱️ {}</span>", human_ts(meta.duration));
    let _ = write!(chips, "<span>💾 {}</span>", human_size(meta.size));
    if meta.width > 0 {
        let _ = write!(chips, "<span>📐 {}x{}</span>", meta.width, meta.height);
    }
    if meta.fps > 0.0 {
        let _ = write!(chips, "<span>🎞️ {:.2} fps</span>", meta.fps);
    }
    if !meta.codec.is_empty() {
        let _ = write!(chips, "<span>⚙️ {}</span>", html_escape::encode_text(&meta.codec));
    }
    if !meta.audio_codecs.is_empty() {
        let browser_compat = ["AAC", "MP3", "OPUS", "VORBIS", "FLAC", "PCM", "MP2"];
        let incompatible = meta.audio_codecs.iter().any(|c| {
            !browser_compat.iter().any(|ok| c.contains(ok))
        });
        let label = meta.audio_codecs.join(", ");
        let icon = if incompatible { "🔇" } else { "🔊" };
        let _ = write!(chips, "<span>{icon} {}</span>", html_escape::encode_text(&label));
    }
    chips
}

fn render_chapters(vid_id: &str, meta: &crate::ffmpeg::VideoMetadata, mode: &str, offset: f64, width: u32) -> String {
    let mut ch_html = String::new();
    for (i, ch) in meta.chapters.iter().enumerate() {
        let img_url = format!("/thumb/{vid_id}/{i}.jpg?mode={mode}&offset={offset}&width={width}");
        let ch_title = html_escape::encode_text(&ch.title);
        let fallback_title = format!("Chapter {}", i + 1);
        let title = if ch.title.is_empty() { fallback_title.as_str() } else { ch_title.as_ref() };
        let dur_fmt = human_ts(ch.end - ch.start);

        let _ = write!(ch_html, r#"
            <div class="chapter-item">
                <img class="chapter-thumb" src="{img_url}" loading="lazy" onclick="lb.openChapter('{img_url}', '/video/{vid_id}', {})">
                <div class="chapter-overlay">
                    <div class="chapter-time">{dur_fmt}</div>
                    <div class="chapter-title" title="{title}">{title}</div>
                </div>
            </div>
            "#, ch.start);
    }
    ch_html
}

fn render_card(v: &VideoEntry, tab_idx: usize, mode: &str, offset: f64, width: u32) -> String {
    let vid_id = &v.id;
    let ch_html = render_chapters(vid_id, &v.meta, mode, offset, width);
    let chips = render_chips(&v.meta);
    let play_url = format!("/video/{vid_id}");
    let rel_path_str = v.rel_path.to_string_lossy();
    let rel_path_esc = html_escape::encode_text(&rel_path_str);
    let rel_path_lower = v.rel_path.to_string_lossy().to_lowercase();
    let rel_path_data_esc = html_escape::encode_text(&rel_path_lower);

    format!(r#"
        <div class="video-card" data-path="{rel_path_data_esc}" data-tab="{tab_idx}">
            <div class="video-header">
                <div class="video-info">
                    <div class="video-title" id="title-{vid_id}">
                        <span class="title-text">{rel_path_esc}</span>
                        <button class="btn-icon-raw" onclick="startRename('{vid_id}')" title="Rename">✏️</button>
                    </div>
                    <div class="video-meta">{chips}</div>
                </div>
                <div class="video-actions">
                    <button class="btn-icon" onclick="openFile('{vid_id}')" title="Open in system player">
                        ▶️ System
                    </button>
                    <button class="btn-icon" onclick="lb.openVideo('{play_url}')" title="Play in browser">
                        🌐 Browser
                    </button>
                    <button class="btn-icon" onclick="openDir('{vid_id}')" title="Open directory">
                        📂 Folder
                    </button>
                </div>
            </div>
            <div class="chapters-grid">
                {ch_html}
            </div>
        </div>
        "#)
}

fn render_tab_bar(videos_len: usize) -> String {
    let tab_count = videos_len.div_ceil(PAGE_SIZE);
    if tab_count <= 1 {
        return String::new();
    }
    let mut html = String::from(r#"<div class="tab-bar">"#);
    for t in 0..tab_count {
        let start = t * PAGE_SIZE + 1;
        let end = ((t + 1) * PAGE_SIZE).min(videos_len);
        let _ = write!(
            html,
            r#"<button class="tab-btn{}" data-tab="{t}">{start}–{end}</button>"#,
            if t == 0 { " active" } else { "" }
        );
    }
    html.push_str("</div>");
    html
}

fn render_controls(mode: &str, offset: f64, width: u32) -> String {
    format!(r#"
    <div class="controls-row">
        <div class="search-box">
            <span class="search-icon">🔍</span>
            <input type="search" id="search-input" placeholder="Search videos..." autocomplete="off">
        </div>

        <form method="get" class="controls">
            <div class="control-group">
                <label>Preview Position</label>
                <select name="mode" onchange="this.form.submit()">
                    <option value="percent" {}>Percent (%)</option>
                    <option value="seconds" {}>Seconds (s)</option>
                </select>
                <input type="number" name="offset" value="{offset}" step="0.5" style="width: 80px">
            </div>
            <div class="control-group">
                <label>Size</label>
                <select name="width" onchange="this.form.submit()">
                    <option value="640" {}>Small (640px)</option>
                    <option value="1280" {}>Medium (1280px)</option>
                    <option value="1920" {}>Large (1920px)</option>
                    <option value="0" {}>Original Size</option>
                </select>
            </div>
            <button type="submit" class="primary">Update</button>
        </form>
    </div>
    "#,
    if mode == "percent" { "selected" } else { "" },
    if mode == "seconds" { "selected" } else { "" },
    if width == 640 { "selected" } else { "" },
    if width == 1280 { "selected" } else { "" },
    if width == 1920 { "selected" } else { "" },
    if width == 0 { "selected" } else { "" }
    )
}

pub fn generate_index_html(videos: &[VideoEntry], count: usize, root_path: &str, mode: &str, offset: f64, width: u32, scanning: bool) -> String {
    if scanning {
        return format!(r#"<!doctype html>
            <html>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta http-equiv="refresh" content="2">
                <title>VidDeck Loading...</title>
                <style>
                    body {{ font-family: sans-serif; background: #111827; color: #f3f4f6; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; }}
                    .loader {{ border: 4px solid #374151; border-top: 4px solid #6366f1; border-radius: 50%; width: 40px; height: 40px; animation: spin 1s linear infinite; margin-bottom: 20px; }}
                    @keyframes spin {{ 0% {{ transform: rotate(0deg); }} 100% {{ transform: rotate(360deg); }} }}
                    .content {{ text-align: center; }}
                </style>
            </head>
            <body>
                <div class="content">
                    <div class="loader" style="margin: 0 auto 20px;"></div>
                    <h2>Scanning library...</h2>
                    <p>{count} videos found</p>
                </div>
            </body>
            </html>
            "#);
    }

    let mut cards = String::new();
    for (i, v) in videos.iter().enumerate() {
        let tab_idx = i / PAGE_SIZE;
        cards.push_str(&render_card(v, tab_idx, mode, offset, width));
    }

    if cards.is_empty() {
        cards.push_str(r#"
        <div class="empty-state" id="empty-state">
            <div class="empty-icon">📭</div>
            <h3>No videos found</h3>
            <p>Try a different directory.</p>
        </div>
        "#);
    }

    cards.push_str(r#"
    <div class="no-results" id="no-results">
        <div class="empty-icon">🔍</div>
        <h3>No results</h3>
        <p>No videos found matching your search.</p>
    </div>
    "#);

    let tab_bar = render_tab_bar(videos.len());
    let controls = render_controls(mode, offset, width);

    format!(r#"<!doctype html>
    <html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>VidDeck</title>
        <link rel="stylesheet" href="/style.css">
    </head>
    <body>
        <div class="container">
            <header>
                <div class="header-content">
                    <h1>VidDeck</h1>
                    <p>{count} videos in {}</p>
                </div>
                {controls}
            </header>
            {tab_bar}
            {cards}
        </div>
        <div id="lightbox" class="lightbox"></div>
        <script src="/script.js"></script>
    </body>
    </html>
    "#, html_escape::encode_text(root_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_ts_seconds_only() {
        assert_eq!(human_ts(0.0), "00:00");
        assert_eq!(human_ts(5.0), "00:05");
        assert_eq!(human_ts(59.0), "00:59");
    }

    #[test]
    fn human_ts_minutes() {
        assert_eq!(human_ts(60.0), "01:00");
        assert_eq!(human_ts(90.0), "01:30");
        assert_eq!(human_ts(3599.0), "59:59");
    }

    #[test]
    fn human_ts_hours() {
        assert_eq!(human_ts(3600.0), "01:00:00");
        assert_eq!(human_ts(7261.0), "02:01:01");
    }

    #[test]
    fn human_ts_negative() {
        // Should clamp to 0
        assert_eq!(human_ts(-5.0), "00:00");
    }

    #[test]
    fn human_size_zero() {
        assert_eq!(human_size(0), "0 B");
    }

    #[test]
    fn human_size_bytes() {
        assert_eq!(human_size(500), "500.0 B");
    }

    #[test]
    fn human_size_kilobytes() {
        assert_eq!(human_size(1024), "1.0 KB");
        assert_eq!(human_size(1536), "1.5 KB");
    }

    #[test]
    fn human_size_megabytes() {
        assert_eq!(human_size(1_048_576), "1.0 MB");
    }

    #[test]
    fn human_size_gigabytes() {
        assert_eq!(human_size(1_073_741_824), "1.0 GB");
    }

    #[test]
    fn human_size_terabytes() {
        assert_eq!(human_size(1_099_511_627_776), "1.0 TB");
    }
}
