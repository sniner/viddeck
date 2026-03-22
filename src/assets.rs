pub const STYLESHEET: &str = r#"
:root {
    --primary: #6366f1;
    --primary-hover: #4f46e5;
    --bg: #f3f4f6;
    --card-bg: #ffffff;
    --text-main: #111827;
    --text-muted: #6b7280;
    --border: #e5e7eb;
    --thumb-bg: #e5e7eb;
    --shadow-sm:    0 1px 3px rgba(0,0,0,0.06), 0 1px 2px rgba(0,0,0,0.04);
    --shadow-md:    0 4px 12px rgba(0,0,0,0.08), 0 2px 4px rgba(0,0,0,0.05);
    --shadow-hover: 0 14px 36px rgba(0,0,0,0.14), 0 5px 14px rgba(0,0,0,0.09);
}

@media (prefers-color-scheme: dark) {
    :root {
        --primary: #818cf8;
        --primary-hover: #6366f1;
        --bg: #111827;
        --card-bg: #1f2937;
        --text-main: #f3f4f6;
        --text-muted: #9ca3af;
        --border: #374151;
        --thumb-bg: #374151;
        --shadow-sm:    0 1px 3px rgba(0,0,0,0.25), 0 1px 2px rgba(0,0,0,0.18);
        --shadow-md:    0 4px 12px rgba(0,0,0,0.35), 0 2px 4px rgba(0,0,0,0.25);
        --shadow-hover: 0 14px 36px rgba(0,0,0,0.55), 0 5px 14px rgba(0,0,0,0.40);
    }
}

* { margin: 0; padding: 0; box-sizing: border-box; }

body {
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
    background: var(--bg);
    color: var(--text-main);
    line-height: 1.5;
    padding: 20px;
}

.container {
    max-width: 1920px;
    margin: 0 auto;
}

/* Header */
header {
    background: var(--card-bg);
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 24px;
    border: 1px solid var(--border);
    box-shadow: var(--shadow-md);
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: 20px;
}

.header-content h1 {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--text-main);
    display: flex;
    align-items: center;
    gap: 10px;
}
.header-content p {
    color: var(--text-muted);
    font-size: 0.8rem;
    margin-top: 4px;
}

/* Controls */
.controls-row {
    display: flex;
    gap: 20px;
    align-items: center;
    flex-wrap: wrap;
}

.controls {
    display: flex;
    gap: 16px;
    align-items: center;
    flex-wrap: wrap;
}

.control-group {
    display: flex;
    align-items: center;
    gap: 8px;
}

/* Search Box */
.search-box {
    position: relative;
}
.search-icon {
    position: absolute;
    left: 10px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-muted);
    font-size: 0.9rem;
    pointer-events: none;
}
input[type="search"] {
    padding-left: 32px;
    width: 200px;
    transition: width 0.2s;
}
input[type="search"]:focus {
    width: 280px;
    outline: 2px solid var(--primary);
    border-color: transparent;
}

label {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--text-muted);
}

select, input {
    padding: 6px 10px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--bg);
    font-size: 0.875rem;
    color: var(--text-main);
}

button.primary {
    background: var(--primary);
    color: white;
    border: none;
    padding: 6px 14px;
    border-radius: 8px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s ease;
}
button.primary:hover { background: var(--primary-hover); }

/* Video Card */
.video-card {
    background: var(--card-bg);
    border-radius: 12px;
    margin-bottom: 24px;
    border: 1px solid var(--border);
    box-shadow: var(--shadow-sm);
    transition: box-shadow 0.25s ease;
}

.video-header {
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
}

.video-info { flex: 1; min-width: 0; }

.video-title {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: 6px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.video-meta {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    font-size: 0.8rem;
    color: var(--text-muted);
}

.meta-chip {
    display: flex;
    align-items: center;
    gap: 6px;
}

.video-actions {
    display: flex;
    gap: 8px;
    margin-left: 20px;
    flex-shrink: 0;
}

.btn-icon {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-muted);
    padding: 4px 8px;
    border-radius: 8px;
    cursor: pointer;
    font-size: 0.8rem;
    display: flex;
    align-items: center;
    gap: 6px;
    transition: all 0.2s;
}
.btn-icon:hover {
    background: var(--bg);
    color: var(--text-main);
    border-color: var(--text-muted);
}

.btn-icon-raw {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    font-size: 0.9rem;
    padding: 2px 6px;
    opacity: 0.6;
    transition: opacity 0.2s;
}
.btn-icon-raw:hover {
    opacity: 1;
    color: var(--primary);
}

.rename-form {
    display: flex;
    gap: 8px;
    align-items: center;
}
.rename-input {
    padding: 4px 8px;
    font-size: 0.9rem;
    border: 1px solid var(--primary);
    border-radius: 4px;
    width: 300px;
}

/* Chapters */
.chapters-grid {
    padding: 20px;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 20px;
}

.chapter-item {
    position: relative;
    border-radius: 8px;
    overflow: hidden;
    background: var(--thumb-bg);
    transition: transform 0.25s ease, box-shadow 0.25s ease;
}
.chapter-item:hover {
    box-shadow: var(--shadow-hover);
    transform: translateY(-5px);
    z-index: 5;
}

.chapter-thumb {
    width: 100%;
    aspect-ratio: 16/9;
    object-fit: cover;
    display: block;
    cursor: pointer;
    background: var(--thumb-bg);
    min-height: 100px;
}

.chapter-overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(to top, rgba(0,0,0,0.95) 0%, rgba(0,0,0,0.7) 50%, transparent 100%);
    padding: 30px 12px 10px;
    color: white;
    pointer-events: none;
}

.chapter-time {
    font-family: monospace;
    font-size: 0.75rem;
    opacity: 0.9;
    margin-bottom: 2px;
}

.chapter-title {
    font-weight: 500;
    font-size: 0.85rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

/* Lightbox */
.lightbox {
    display: none;
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.95);
    z-index: 1000;
    justify-content: center;
    align-items: center;
    padding: 40px;
    flex-direction: column;
}
.lightbox.active { display: flex; }
.lightbox img {
    max-width: 100%;
    max-height: 85vh;
    border-radius: 8px;
    box-shadow: 0 0 50px rgba(0,0,0,0.5);
    cursor: pointer;
}
.lightbox-close {
    position: absolute;
    top: 20px;
    right: 20px;
    color: white;
    font-size: 30px;
    cursor: pointer;
    background: none;
    border: none;
    opacity: 0.7;
    transition: opacity 0.2s;
}
.lightbox-close:hover { opacity: 1; }

.lightbox-actions {
    display: flex;
    gap: 12px;
    margin-top: 16px;
}
.lightbox-btn {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background: rgba(255,255,255,0.15);
    border: 1.5px solid rgba(255,255,255,0.35);
    backdrop-filter: blur(8px);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.2s ease, background 0.2s ease;
}
.lightbox-btn:hover {
    background: rgba(255,255,255,0.28);
    transform: scale(1.1);
}

.lightbox video {
    max-width: 100%;
    max-height: 85vh;
    outline: none;
}

/* Empty State */
.empty-state { text-align: center; padding: 80px; color: var(--text-muted); }
.empty-icon { font-size: 48px; margin-bottom: 16px; }
.no-results { text-align: center; padding: 40px; color: var(--text-muted); display: none; }

/* Tab Bar */
.tab-bar {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 20px;
}

.tab-btn {
    background: var(--card-bg);
    border: 1px solid var(--border);
    color: var(--text-muted);
    padding: 6px 14px;
    border-radius: 8px;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 500;
    transition: all 0.15s ease;
    box-shadow: var(--shadow-sm);
}

.tab-btn:hover {
    color: var(--text-main);
    border-color: var(--text-muted);
}

.tab-btn.active {
    background: var(--primary);
    color: white;
    border-color: var(--primary);
}

/* Scanning spinner */
.scanning-screen {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 60vh;
}
.scanning-content {
    text-align: center;
}
.loader {
    border: 4px solid var(--border);
    border-top: 4px solid var(--primary);
    border-radius: 50%;
    width: 40px;
    height: 40px;
    animation: spin 1s linear infinite;
    margin: 0 auto 20px;
}
@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

"#;

pub const JAVASCRIPT: &str = r#"
'use strict';

const PAGE_SIZE = 50;
const CONFIG_KEY = 'viddeck_config';
const BROWSER_COMPAT_AUDIO = ['AAC', 'MP3', 'OPUS', 'VORBIS', 'FLAC', 'PCM', 'MP2'];

const APP = {
    videos: {},
    root: '',
    scanning: false,
    remote: false,
    settings: { mode: 'percent', offset: 50, width: 1280 },
    searchTerm: '',
    activeTab: 0,
};

// --- Utilities ---

function humanTs(sec) {
    const total = Math.max(0, sec);
    const h = Math.floor(total / 3600);
    const m = Math.floor((total % 3600) / 60);
    const s = Math.floor(total % 60);
    const pad = n => String(n).padStart(2, '0');
    return h > 0 ? `${pad(h)}:${pad(m)}:${pad(s)}` : `${pad(m)}:${pad(s)}`;
}

function humanSize(bytes) {
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
    let size = bytes;
    for (const unit of units) {
        if (size < 1024) return `${size.toFixed(1)} ${unit}`;
        size /= 1024;
    }
    return `${size.toFixed(1)} EB`;
}

function el(tag, attrs, ...children) {
    const e = document.createElement(tag);
    if (attrs) {
        for (const [k, v] of Object.entries(attrs)) {
            if (k === 'className') e.className = v;
            else if (k.startsWith('on')) e.addEventListener(k.slice(2).toLowerCase(), v);
            else if (k === 'style' && typeof v === 'object') Object.assign(e.style, v);
            else e.setAttribute(k, v);
        }
    }
    for (const c of children) {
        if (c == null) continue;
        e.appendChild(typeof c === 'string' ? document.createTextNode(c) : c);
    }
    return e;
}

// --- Settings ---

function loadSettings() {
    try {
        const stored = JSON.parse(localStorage.getItem(CONFIG_KEY));
        if (stored) {
            if (stored.mode) APP.settings.mode = stored.mode;
            if (stored.offset != null) APP.settings.offset = Number(stored.offset);
            if (stored.width != null) APP.settings.width = Number(stored.width);
        }
    } catch(e) { /* ignore */ }
}

function saveSettings() {
    localStorage.setItem(CONFIG_KEY, JSON.stringify(APP.settings));
}

// --- API ---

let fetchTimer = null;
let fetchInFlight = false;

function scheduleFetch(delayMs) {
    if (fetchTimer) clearTimeout(fetchTimer);
    fetchTimer = setTimeout(fetchVideos, delayMs);
}

async function fetchVideos() {
    if (fetchTimer) { clearTimeout(fetchTimer); fetchTimer = null; }
    if (fetchInFlight) return;
    fetchInFlight = true;
    try {
        const res = await fetch('/api/videos');
        const data = await res.json();
        const wasScanning = APP.scanning;
        APP.videos = data.videos;
        APP.root = data.root;
        APP.scanning = data.scanning;
        APP.remote = data.remote;

        if (wasScanning && !APP.scanning) {
            // Scan just finished — full render with final sorted list
            render();
        } else if (APP.scanning && document.getElementById('cards-container')) {
            // Still scanning but already showing content — just update header count
            updateScanProgress();
            scheduleFetch(2000);
        } else {
            // First render, or non-scanning refresh (SSE from watcher, rename, etc.)
            render();
        }
    } catch(e) {
        console.error('Failed to fetch videos:', e);
    } finally {
        fetchInFlight = false;
    }
}

async function apiCall(endpoint, id) {
    try {
        const res = await fetch(`/api/${endpoint}?id=${encodeURIComponent(id)}`, { method: 'POST' });
        if (!res.ok) {
            alert(`Command failed: ${await res.text()}`);
        }
    } catch (e) {
        alert(`Error: ${e}`);
    }
}

async function submitRename(id) {
    const input = document.getElementById(`input-${id}`);
    if (!input) return;

    const newName = input.value.trim();
    if (!newName) return;

    try {
        const formData = new URLSearchParams();
        formData.append('id', id);
        formData.append('new_name', newName);
        const res = await fetch('/api/rename', {
            method: 'POST',
            body: formData
        });

        if (res.ok) {
            const result = await res.json();
            // Update local state
            const entry = APP.videos[result.old_id];
            if (entry) {
                delete APP.videos[result.old_id];
                entry.rel_path = result.rel_path;
                APP.videos[result.new_id] = entry;
            }
            render();
        } else {
            alert(`Rename failed: ${await res.text()}`);
        }
    } catch (e) {
        alert(`Error: ${e}`);
    }
}

// --- Rename UI ---

const renameState = {};

function startRename(id) {
    const container = document.getElementById(`title-${id}`);
    if (!container || renameState[id]) return;

    renameState[id] = true;
    const currentName = container.querySelector('.title-text').textContent;

    container.textContent = '';
    const form = el('div', { className: 'rename-form', onClick: e => e.stopPropagation() });
    const input = el('input', { type: 'text', className: 'rename-input', id: `input-${id}`, value: currentName });
    const saveBtn = el('button', { className: 'btn-icon-raw', title: 'Save', onClick: () => submitRename(id) }, '\u2705');
    const cancelBtn = el('button', { className: 'btn-icon-raw', title: 'Cancel', onClick: () => cancelRename(id) }, '\u274c');

    input.addEventListener('keydown', e => {
        if (e.key === 'Enter') submitRename(id);
        if (e.key === 'Escape') cancelRename(id);
    });

    form.appendChild(input);
    form.appendChild(saveBtn);
    form.appendChild(cancelBtn);
    container.appendChild(form);
    input.focus();
    input.select();
}

function cancelRename(id) {
    delete renameState[id];
    render();
}

// --- Thumbnail URL ---

function thumbUrl(vidId, idx) {
    const s = APP.settings;
    return `/thumb/${vidId}/${idx}.jpg?mode=${s.mode}&offset=${s.offset}&width=${s.width}`;
}

// --- Transcode helpers ---

const BROWSER_COMPAT_VIDEO = ['H264', 'VP8', 'VP9', 'AV1'];

function needsTranscode(vidId) {
    const v = APP.videos[vidId];
    if (!v) return false;
    const badVideo = v.codec && !BROWSER_COMPAT_VIDEO.some(ok => v.codec.toUpperCase().includes(ok));
    const badAudio = v.audio_codecs && v.audio_codecs.length > 0
        && v.audio_codecs.some(c => !BROWSER_COMPAT_AUDIO.some(ok => c.includes(ok)));
    return badVideo || badAudio;
}
function videoSrc(vidId) {
    return needsTranscode(vidId) ? `/video/${vidId}/transcode` : `/video/${vidId}`;
}
function videoSrcAt(vidId, t) {
    return needsTranscode(vidId) ? `/video/${vidId}/transcode?t=${Math.floor(t)}` : `/video/${vidId}`;
}

// --- Rendering ---

function renderChips(video) {
    const frag = document.createDocumentFragment();
    const chip = text => { const s = el('span', null, text); frag.appendChild(s); };

    chip(`\u23f1\ufe0f ${humanTs(video.duration)}`);
    chip(`\ud83d\udcbe ${humanSize(video.size)}`);
    if (video.width > 0) chip(`\ud83d\udcd0 ${video.width}x${video.height}`);
    if (video.fps > 0) chip(`\ud83c\udf9e\ufe0f ${video.fps.toFixed(2)} fps`);
    if (video.codec) chip(`\u2699\ufe0f ${video.codec}`);
    if (video.audio_codecs && video.audio_codecs.length > 0) {
        const incompatible = video.audio_codecs.some(c => !BROWSER_COMPAT_AUDIO.some(ok => c.includes(ok)));
        const icon = incompatible ? '\ud83d\udd07' : '\ud83d\udd0a';
        chip(`${icon} ${video.audio_codecs.join(', ')}`);
    }
    return frag;
}

function renderChapterItem(vidId, idx, chapter) {
    const dur = humanTs(chapter.end - chapter.start);
    const title = chapter.title || `Chapter ${idx + 1}`;

    const item = el('div', { className: 'chapter-item' });
    const img = el('img', {
        className: 'chapter-thumb',
        src: thumbUrl(vidId, idx),
        loading: 'lazy',
    });
    // Read img.src at click time so settings changes are reflected
    img.addEventListener('click', () => lb.openChapter(img.src, vidId, chapter.start));
    item.appendChild(img);

    const overlay = el('div', { className: 'chapter-overlay' });
    overlay.appendChild(el('div', { className: 'chapter-time' }, dur));
    const titleEl = el('div', { className: 'chapter-title', title: title });
    titleEl.textContent = title;
    overlay.appendChild(titleEl);
    item.appendChild(overlay);

    return item;
}

function renderCard(id, video, tabIdx) {
    const card = el('div', { className: 'video-card', 'data-tab': String(tabIdx), 'data-path': video.rel_path.toLowerCase() });

    // Header
    const header = el('div', { className: 'video-header' });
    const info = el('div', { className: 'video-info' });

    const titleDiv = el('div', { className: 'video-title', id: `title-${id}` });
    const titleSpan = el('span', { className: 'title-text' });
    titleSpan.textContent = video.rel_path;
    titleDiv.appendChild(titleSpan);
    titleDiv.appendChild(el('button', { className: 'btn-icon-raw', title: 'Rename', onClick: () => startRename(id) }, '\u270f\ufe0f'));
    info.appendChild(titleDiv);

    const meta = el('div', { className: 'video-meta' });
    meta.appendChild(renderChips(video));
    info.appendChild(meta);
    header.appendChild(info);

    const actions = el('div', { className: 'video-actions' });
    if (APP.remote) {
        actions.appendChild(el('button', { className: 'btn-icon', onClick: () => lb.openVideo(videoSrc(id)), title: 'Play' }, '\u25b6\ufe0f'));
    } else {
        actions.appendChild(el('button', { className: 'btn-icon', onClick: () => apiCall('open_file', id), title: 'Open in system player' }, '\u25b6\ufe0f System'));
        actions.appendChild(el('button', { className: 'btn-icon', onClick: () => lb.openVideo(videoSrc(id)), title: 'Play in browser' }, '\ud83c\udf10 Browser'));
        actions.appendChild(el('button', { className: 'btn-icon', onClick: () => apiCall('open_dir', id), title: 'Open directory' }, '\ud83d\udcc2 Folder'));
    }
    header.appendChild(actions);
    card.appendChild(header);

    // Chapters grid
    const grid = el('div', { className: 'chapters-grid' });
    video.chapters.forEach((ch, i) => grid.appendChild(renderChapterItem(id, i, ch)));
    card.appendChild(grid);

    return card;
}

function renderTabBar(count) {
    const tabCount = Math.ceil(count / PAGE_SIZE);
    if (tabCount <= 1) return null;

    const bar = el('div', { className: 'tab-bar' });
    for (let t = 0; t < tabCount; t++) {
        const start = t * PAGE_SIZE + 1;
        const end = Math.min((t + 1) * PAGE_SIZE, count);
        const btn = el('button', {
            className: 'tab-btn' + (t === APP.activeTab ? ' active' : ''),
            'data-tab': String(t),
            onClick: () => { APP.activeTab = t; sessionStorage.setItem('viddeck_tab', t); showTab(); },
        }, `${start}\u2013${end}`);
        bar.appendChild(btn);
    }
    return bar;
}

function renderControls() {
    const s = APP.settings;
    const row = el('div', { className: 'controls-row' });

    // Search
    const searchBox = el('div', { className: 'search-box' });
    searchBox.appendChild(el('span', { className: 'search-icon' }, '\ud83d\udd0d'));
    const searchInput = el('input', { type: 'search', id: 'search-input', placeholder: 'Search videos...', autocomplete: 'off' });
    searchInput.value = APP.searchTerm;
    searchInput.addEventListener('input', e => {
        APP.searchTerm = e.target.value;
        sessionStorage.setItem('viddeck_search', APP.searchTerm);
        filterAndShow();
    });
    searchBox.appendChild(searchInput);
    row.appendChild(searchBox);

    // Controls div
    const controls = el('div', { className: 'controls' });

    // Mode
    const modeGroup = el('div', { className: 'control-group' });
    modeGroup.appendChild(el('label', null, 'Preview Position'));
    const modeSelect = el('select');
    modeSelect.appendChild(el('option', { value: 'percent', ...(s.mode === 'percent' ? { selected: '' } : {}) }, 'Percent (%)'));
    modeSelect.appendChild(el('option', { value: 'seconds', ...(s.mode === 'seconds' ? { selected: '' } : {}) }, 'Seconds (s)'));
    modeSelect.addEventListener('change', e => { APP.settings.mode = e.target.value; saveSettings(); updateThumbnails(); });
    modeGroup.appendChild(modeSelect);

    // Offset
    const offsetInput = el('input', { type: 'number', value: String(s.offset), step: '0.5', style: { width: '80px' } });
    offsetInput.addEventListener('change', e => { APP.settings.offset = Number(e.target.value); saveSettings(); updateThumbnails(); });
    modeGroup.appendChild(offsetInput);
    controls.appendChild(modeGroup);

    // Width
    const widthGroup = el('div', { className: 'control-group' });
    widthGroup.appendChild(el('label', null, 'Size'));
    const widthSelect = el('select');
    for (const [val, label] of [['640','Small (640px)'],['1280','Medium (1280px)'],['1920','Large (1920px)'],['0','Original Size']]) {
        widthSelect.appendChild(el('option', { value: val, ...(String(s.width) === val ? { selected: '' } : {}) }, label));
    }
    widthSelect.addEventListener('change', e => { APP.settings.width = Number(e.target.value); saveSettings(); updateThumbnails(); });
    widthGroup.appendChild(widthSelect);
    controls.appendChild(widthGroup);

    row.appendChild(controls);
    return row;
}

// --- Update thumbnails without re-render ---

function updateThumbnails() {
    document.querySelectorAll('.chapter-thumb').forEach(img => {
        const oldSrc = img.getAttribute('src');
        if (!oldSrc) return;
        // Parse vidId and idx from URL: /thumb/{vidId}/{idx}.jpg?...
        const match = oldSrc.match(/^\/thumb\/([^/]+)\/(\d+)\.jpg/);
        if (match) {
            img.src = thumbUrl(match[1], match[2]);
        }
    });
}

// --- Tab / Filter logic ---

let sortedIds = [];
let filteredIds = [];

function getSortedIds() {
    return Object.entries(APP.videos)
        .sort((a, b) => a[1].rel_path.localeCompare(b[1].rel_path))
        .map(e => e[0]);
}

function filterAndShow() {
    const term = APP.searchTerm.toLowerCase().trim();
    const noResults = document.getElementById('no-results');
    const emptyState = document.getElementById('empty-state');
    const tabBar = document.querySelector('.tab-bar');

    if (!term) {
        filteredIds = sortedIds;
    } else {
        filteredIds = sortedIds.filter(id => {
            const v = APP.videos[id];
            return v && v.rel_path.toLowerCase().includes(term);
        });
    }

    // Rebuild tab bar for filtered results
    if (tabBar) tabBar.remove();
    const container = document.querySelector('.container');
    const cardsStart = document.getElementById('cards-container');

    const maxTab = Math.max(0, Math.ceil(filteredIds.length / PAGE_SIZE) - 1);
    if (term) APP.activeTab = 0;
    if (APP.activeTab > maxTab) APP.activeTab = 0;

    if (filteredIds.length > PAGE_SIZE) {
        const newBar = renderTabBar(filteredIds.length);
        if (newBar) container.insertBefore(newBar, cardsStart);
    }

    showTab();

    if (noResults) noResults.style.display = filteredIds.length === 0 && sortedIds.length > 0 ? 'block' : 'none';
    if (emptyState) emptyState.style.display = sortedIds.length === 0 && !APP.scanning ? '' : 'none';
}

function showTab() {
    const start = APP.activeTab * PAGE_SIZE;
    const pageIds = filteredIds.slice(start, start + PAGE_SIZE);

    const container = document.getElementById('cards-container');
    container.textContent = '';

    pageIds.forEach((id, i) => {
        const video = APP.videos[id];
        if (video) container.appendChild(renderCard(id, video, APP.activeTab));
    });

    // Update tab bar active state
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.classList.toggle('active', parseInt(btn.dataset.tab) === APP.activeTab);
    });
}

// --- Scanning progress (no re-render) ---

function updateScanProgress() {
    const count = Object.keys(APP.videos).length;
    const subtitle = document.getElementById('header-subtitle');
    if (subtitle) {
        subtitle.textContent = `Scanning\u2026 ${count} videos found in ${APP.root}`;
    }
}

// --- Main render ---

function render() {
    const app = document.getElementById('app');
    app.textContent = '';

    if (APP.scanning && Object.keys(APP.videos).length === 0) {
        const screen = el('div', { className: 'scanning-screen' });
        const content = el('div', { className: 'scanning-content' });
        content.appendChild(el('div', { className: 'loader' }));
        content.appendChild(el('h2', null, 'Scanning library...'));
        screen.appendChild(content);
        app.appendChild(screen);
        scheduleFetch(2000);
        return;
    }

    const container = el('div', { className: 'container' });

    // Header
    const header = el('header');
    const headerContent = el('div', { className: 'header-content' });
    headerContent.appendChild(el('h1', null, 'VidDeck'));
    const count = Object.keys(APP.videos).length;
    const subtitle = el('p', { id: 'header-subtitle' });
    if (APP.scanning) {
        subtitle.textContent = `Scanning\u2026 ${count} videos found in ${APP.root}`;
    } else {
        subtitle.textContent = `${count} videos in ${APP.root}`;
    }
    headerContent.appendChild(subtitle);
    header.appendChild(headerContent);
    header.appendChild(renderControls());
    container.appendChild(header);

    // Cards container
    const cardsContainer = el('div', { id: 'cards-container' });
    container.appendChild(cardsContainer);

    // Empty state (hidden by default)
    const emptyState = el('div', { className: 'empty-state', id: 'empty-state', style: { display: 'none' } });
    emptyState.appendChild(el('div', { className: 'empty-icon' }, '\ud83d\udced'));
    emptyState.appendChild(el('h3', null, 'No videos found'));
    emptyState.appendChild(el('p', null, 'Try a different directory.'));
    container.appendChild(emptyState);

    // No results (hidden by default)
    const noResults = el('div', { className: 'no-results', id: 'no-results' });
    noResults.appendChild(el('div', { className: 'empty-icon' }, '\ud83d\udd0d'));
    noResults.appendChild(el('h3', null, 'No results'));
    noResults.appendChild(el('p', null, 'No videos found matching your search.'));
    container.appendChild(noResults);

    app.appendChild(container);

    // Restore search term
    APP.searchTerm = sessionStorage.getItem('viddeck_search') || '';
    const savedTab = parseInt(sessionStorage.getItem('viddeck_tab') || '0');
    APP.activeTab = savedTab;

    // Compute sorted IDs and filter
    sortedIds = getSortedIds();
    filterAndShow();

    // If still scanning, poll for updates (cards stay stable, only header updates)
    if (APP.scanning) {
        scheduleFetch(2000);
    }
}

// --- Lightbox ---

const lb = {
    el: null,
    init() {
        this.el = document.getElementById('lightbox');
        this.el.addEventListener('click', (e) => {
            if (e.target.closest('video')) return;
            this.close();
        });
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') this.close();
        });
    },
    _clear() {
        this.el.innerHTML = '';
    },
    _addCloseBtn() {
        const btn = document.createElement('button');
        btn.className = 'lightbox-close';
        btn.textContent = '\u00d7';
        this.el.appendChild(btn);
    },
    _createVideo(src) {
        const video = document.createElement('video');
        video.controls = true;
        video.autoplay = true;
        video.addEventListener('click', (e) => e.stopPropagation());
        const source = document.createElement('source');
        source.src = src;
        source.type = 'video/mp4';
        video.appendChild(source);
        return video;
    },
    openImage(src) {
        this._clear();
        const img = document.createElement('img');
        img.src = src;
        img.title = 'Click to close';
        this.el.appendChild(img);
        this._addCloseBtn();
        this.el.classList.add('active');
    },
    openChapter(imgSrc, vidId, startSec) {
        const t = Math.floor(startSec);
        this._clear();
        const img = document.createElement('img');
        img.src = imgSrc;
        img.title = 'Click to close';
        this.el.appendChild(img);

        const actions = document.createElement('div');
        actions.className = 'lightbox-actions';
        const playBtn = document.createElement('button');
        playBtn.className = 'lightbox-btn';
        playBtn.title = 'Im Browser abspielen';
        playBtn.innerHTML = '<svg viewBox="0 0 24 24" width="22" height="22" fill="white"><path d="M8 5v14l11-7z"/></svg>';
        const playSrc = videoSrcAt(vidId, startSec);
        playBtn.addEventListener('click', (e) => { e.stopPropagation(); lb.openVideoAt(playSrc, t); });
        actions.appendChild(playBtn);
        this.el.appendChild(actions);

        this._addCloseBtn();
        this.el.classList.add('active');
    },
    openVideo(src) {
        this._clear();
        this.el.appendChild(this._createVideo(src));
        this._addCloseBtn();
        this.el.classList.add('active');
    },
    openVideoAt(src, t) {
        this._clear();
        const v = this._createVideo(src);
        this.el.appendChild(v);
        this._addCloseBtn();
        this.el.classList.add('active');
        // Only seek client-side for direct file URLs (not transcoded streams)
        if (!src.includes('/transcode')) {
            v.addEventListener('loadedmetadata', () => { v.currentTime = t; });
        }
    },
    close() {
        this.el.classList.remove('active');
        this.el.innerHTML = '';
    }
};

// --- Init ---

document.addEventListener('DOMContentLoaded', () => {
    lb.init();
    loadSettings();

    // Restore search from session
    APP.searchTerm = sessionStorage.getItem('viddeck_search') || '';

    fetchVideos();

    // SSE for live updates
    const evtSource = new EventSource('/api/events');
    evtSource.addEventListener('message', (e) => {
        if (e.data === 'refresh') {
            console.log('Filesystem change detected, fetching updates...');
            fetchVideos();
        }
    });
});
"#;
