use crate::state::VideoEntry;

pub fn human_ts(sec: f64) -> String {
    let m = ((sec % 3600.0) / 60.0).floor() as u64;
    let s = sec % 60.0;
    let h = (sec / 3600.0).floor() as u64;
    if h > 0 {
        format!("{:02}:{:02}:{:02.0}", h, m, s)
    } else {
        format!("{:02}:{:02.0}", m, s)
    }
}

pub fn human_size(size_bytes: u64) -> String {
    if size_bytes == 0 {
        return "0 B".to_string();
    }
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size_bytes as f64;
    for unit in units {
        if size < 1024.0 {
            return format!("{:.1} {}", size, unit);
        }
        size /= 1024.0;
    }
    format!("{:.1} TB", size)
}

pub fn generate_index_html(videos: &[VideoEntry], count: usize, root_path: &str, mode: &str, value: f64, width: u32, scanning: bool) -> String {
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
                    <p>{} videos found</p>
                </div>
            </body>
            </html>
            "#, count);
    }

    let mut cards = String::new();
    
    for v in videos {
        let meta = &v.meta;
        let vid_id = &v.id;
        
        let mut ch_html = String::new();
        for (i, ch) in meta.chapters.iter().enumerate() {
            let img_url = format!("/thumb/{}/{}.jpg?mode={}&value={}&width={}", vid_id, i, mode, value, width);
            let ch_title = html_escape::encode_text(&ch.title);
            let fallback_title = format!("Chapter {}", i + 1);
            let title = if ch.title.is_empty() { fallback_title.as_str() } else { ch_title.as_ref() };
            let dur_fmt = human_ts(ch.end - ch.start);
            
            ch_html.push_str(&format!(r#"
            <div class="chapter-item">
                <img class="chapter-thumb" src="{}" loading="lazy" onclick="lb.openImage('{}')">
                <div class="chapter-overlay">
                    <div class="chapter-time">{}</div>
                    <div class="chapter-title" title="{}">{}</div>
                </div>
            </div>
            "#, img_url, img_url, dur_fmt, title, title));
        }
        
        // Chips
        let mut chips = String::new();
        chips.push_str(&format!("<span>⏱️ {}</span>", human_ts(meta.duration)));
        chips.push_str(&format!("<span>💾 {}</span>", human_size(meta.size)));
        if meta.width > 0 {
            chips.push_str(&format!("<span>📐 {}x{}</span>", meta.width, meta.height));
        }
        if meta.fps > 0.0 {
            chips.push_str(&format!("<span>🎞️ {:.2} fps</span>", meta.fps));
        }
        if !meta.codec.is_empty() {
            chips.push_str(&format!("<span>⚙️ {}</span>", html_escape::encode_text(&meta.codec)));
        }

        let play_url = format!("/video/{}", vid_id);
        let rel_path_str = v.rel_path.to_string_lossy();
        let rel_path_esc = html_escape::encode_text(&rel_path_str);
        let rel_path_lower = v.rel_path.to_string_lossy().to_lowercase();
        let rel_path_data_esc = html_escape::encode_text(&rel_path_lower);

        cards.push_str(&format!(r#"
        <div class="video-card" data-path="{}">
            <div class="video-header">
                <div class="video-info">
                    <div class="video-title" id="title-{}">
                        <span class="title-text">{}</span>
                        <button class="btn-icon-raw" onclick="startRename('{}')" title="Rename">✏️</button>
                    </div>
                    <div class="video-meta">{}</div>
                </div>
                <div class="video-actions">
                    <button class="btn-icon" onclick="openFile('{}')" title="Open in system player">
                        ▶️ System
                    </button>
                    <button class="btn-icon" onclick="lb.openVideo('{}')" title="Play in browser">
                        🌐 Browser
                    </button>
                    <button class="btn-icon" onclick="openDir('{}')" title="Open directory">
                        📂 Folder
                    </button>
                </div>
            </div>
            <div class="chapters-grid">
                {}
            </div>
        </div>
        "#, rel_path_data_esc, vid_id, rel_path_esc, vid_id, chips, vid_id, play_url, vid_id, ch_html));
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
    
    // Add no results hidden div
    cards.push_str(r#"
    <div class="no-results" id="no-results">
        <div class="empty-icon">🔍</div>
        <h3>No results</h3>
        <p>No videos found matching your search.</p>
    </div>
    "#);

    let controls = format!(r#"
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
                <input type="number" name="value" value="{}" step="0.5" style="width: 80px">
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
    value,
    if width == 640 { "selected" } else { "" },
    if width == 1280 { "selected" } else { "" },
    if width == 1920 { "selected" } else { "" },
    if width == 0 { "selected" } else { "" }
    );

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
                    <p>{} videos in {}</p>
                </div>
                {}
            </header>
            {}
        </div>
        <div id="lightbox" class="lightbox"></div>
        <script src="/script.js"></script>
    </body>
    </html>
    "#, count, html_escape::encode_text(root_path), controls, cards)
}
