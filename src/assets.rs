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

"#;

pub const JAVASCRIPT: &str = r#"
async function apiCall(endpoint, id) {
    try {
        const res = await fetch(`/api/${endpoint}?id=${id}`, { method: 'POST' });
        if (res.ok) {
            console.log(`${endpoint} success`);
        } else {
            alert(`Command failed: ${await res.text()}`);
        }
    } catch (e) {
        alert(`Error: ${e}`);
    }
}

function openFile(id) { apiCall('open_file', id); }
function openDir(id) { apiCall('open_dir', id); }

const renameState = {};

function startRename(id) {
    const container = document.getElementById(`title-${id}`);
    if (!container || renameState[id]) return;

    // Save current content
    renameState[id] = container.innerHTML;
    
    // Get current text content
    const currentName = container.querySelector('.title-text').textContent;

    container.innerHTML = `
        <div class="rename-form" onclick="event.stopPropagation()">
            <input type="text" class="rename-input" id="input-${id}" value="${currentName}">
            <button class="btn-icon-raw" onclick="submitRename('${id}')" title="Save">✅</button>
            <button class="btn-icon-raw" onclick="cancelRename('${id}')" title="Cancel">❌</button>
        </div>
    `;
    
    const input = document.getElementById(`input-${id}`);
    input.focus();
    input.select();
    
    input.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') submitRename(id);
        if (e.key === 'Escape') cancelRename(id);
    });
}

function cancelRename(id) {
    const container = document.getElementById(`title-${id}`);
    if (container && renameState[id]) {
        container.innerHTML = renameState[id];
        delete renameState[id];
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
            window.location.reload();
        } else {
            alert(`Rename failed: ${await res.text()}`);
        }
    } catch (e) {
        alert(`Error: ${e}`);
    }
}

// Persistent Settings
const CONFIG_KEY = 'viddeck_config';

function initSettings() {
    const urlParams = new URLSearchParams(window.location.search);
    const hasParams = urlParams.has('mode') || urlParams.has('offset') || urlParams.has('width');
    
    if (hasParams) {
        // Save current state
        const settings = {
            mode: urlParams.get('mode'),
            offset: urlParams.get('offset'),
            width: urlParams.get('width')
        };
        localStorage.setItem(CONFIG_KEY, JSON.stringify(settings));
    } else {
        // Restore state if clean load
        try {
            const stored = JSON.parse(localStorage.getItem(CONFIG_KEY));
            if (stored) {
                const newUrl = new URL(window.location);
                if (stored.mode) newUrl.searchParams.set('mode', stored.mode);
                if (stored.offset) newUrl.searchParams.set('offset', stored.offset);
                if (stored.width) newUrl.searchParams.set('width', stored.width);
                window.location.replace(newUrl);
            }
        } catch(e) {}
    }
}

// Tab Pagination
const TAB_PAGE_SIZE = 50;

function initTabs() {
    const tabBar = document.querySelector('.tab-bar');
    if (!tabBar) return;

    const allCards = Array.from(document.querySelectorAll('.video-card'));
    const originalHTML = tabBar.innerHTML;
    const originalTabCount = tabBar.querySelectorAll('.tab-btn').length;

    let activeTab = parseInt(sessionStorage.getItem('viddeck_tab') || '0');
    if (activeTab >= originalTabCount) activeTab = 0;

    let searchMatches = null; // null = normal mode, Array = search mode

    function setActiveButton(n) {
        tabBar.querySelectorAll('.tab-btn').forEach(btn => {
            btn.classList.toggle('active', parseInt(btn.dataset.tab) === n);
        });
    }

    function showNormalTab(n) {
        activeTab = n;
        sessionStorage.setItem('viddeck_tab', n);
        allCards.forEach(card => {
            card.style.display = parseInt(card.dataset.tab) === n ? '' : 'none';
        });
        setActiveButton(n);
    }

    function showSearchPage(n) {
        const start = n * TAB_PAGE_SIZE;
        allCards.forEach(card => card.style.display = 'none');
        searchMatches.slice(start, start + TAB_PAGE_SIZE).forEach(card => card.style.display = '');
        setActiveButton(n);
    }

    function buildSearchTabs(count, page) {
        const pageCount = Math.ceil(count / TAB_PAGE_SIZE);
        tabBar.innerHTML = '';
        for (let t = 0; t < pageCount; t++) {
            const start = t * TAB_PAGE_SIZE + 1;
            const end = Math.min((t + 1) * TAB_PAGE_SIZE, count);
            const btn = document.createElement('button');
            btn.className = 'tab-btn' + (t === page ? ' active' : '');
            btn.dataset.tab = t;
            btn.textContent = `${start}\u2013${end}`;
            tabBar.appendChild(btn);
        }
    }

    tabBar.addEventListener('click', e => {
        const btn = e.target.closest('.tab-btn');
        if (!btn) return;
        const n = parseInt(btn.dataset.tab);
        if (searchMatches !== null) {
            showSearchPage(n);
        } else {
            showNormalTab(n);
        }
    });

    showNormalTab(activeTab);

    window._tabEnterSearch = function(matches) {
        searchMatches = matches;
        if (matches.length === 0) {
            tabBar.style.display = 'none';
            allCards.forEach(card => card.style.display = 'none');
        } else {
            tabBar.style.display = '';
            buildSearchTabs(matches.length, 0);
            showSearchPage(0);
        }
    };

    window._tabExitSearch = function() {
        searchMatches = null;
        tabBar.style.display = '';
        tabBar.innerHTML = originalHTML;
        showNormalTab(activeTab);
    };
}

// Live Search
function initSearch() {
    const searchInput = document.getElementById('search-input');
    if (!searchInput) return;

    const allCards = Array.from(document.querySelectorAll('.video-card'));

    function performSearch(term) {
        const lowerTerm = term.toLowerCase().trim();
        const noResults = document.getElementById('no-results');

        if (lowerTerm) {
            const matches = allCards.filter(card => {
                const title = card.querySelector('.video-title').textContent.toLowerCase();
                const relPath = card.getAttribute('data-path') || '';
                return title.includes(lowerTerm) || relPath.includes(lowerTerm);
            });

            if (window._tabEnterSearch) {
                window._tabEnterSearch(matches);
            } else {
                allCards.forEach(card => card.style.display = 'none');
                matches.forEach(card => card.style.display = '');
            }

            if (noResults) noResults.style.display = matches.length === 0 ? 'block' : 'none';
        } else {
            if (window._tabExitSearch) {
                window._tabExitSearch();
            } else {
                allCards.forEach(card => card.style.display = '');
            }
            if (noResults) noResults.style.display = 'none';
        }

        sessionStorage.setItem('viddeck_search', term);
    }

    searchInput.addEventListener('input', (e) => performSearch(e.target.value));

    const stored = sessionStorage.getItem('viddeck_search');
    if (stored) {
        searchInput.value = stored;
        performSearch(stored);
    }
}


// Lightbox — uses DOM APIs to avoid XSS via innerHTML
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
    openChapter(imgSrc, playUrl, startSec) {
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
        playBtn.addEventListener('click', (e) => { e.stopPropagation(); lb.openVideoAt(playUrl, t); });
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
        v.id = 'lb-video';
        this.el.appendChild(v);
        this._addCloseBtn();
        this.el.classList.add('active');
        v.addEventListener('loadedmetadata', () => { v.currentTime = t; });
    },
    close() {
        this.el.classList.remove('active');
        this.el.innerHTML = '';
    }
};

document.addEventListener('DOMContentLoaded', () => {
    lb.init();
    initSettings();
    initTabs();
    initSearch();
    
    // Connect to SSE for auto-refresh
    const evtSource = new EventSource("/api/events");
    evtSource.addEventListener("message", (e) => {
        if (e.data === "refresh") {
            console.log("Filesystem change detected, refreshing...");
            window.location.reload();
        }
    });
});
"#;
