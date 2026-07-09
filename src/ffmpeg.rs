use std::path::Path;
use std::process::Stdio;
use std::sync::OnceLock;
use tokio::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VideoMetadata {
    pub duration: f64,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub codec: String,
    pub audio_codecs: Vec<String>,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub start: f64,
    pub end: f64,
    pub title: String,
}

#[derive(Deserialize)]
struct FFProbeOutput {
    format: Option<FFProbeFormat>,
    streams: Option<Vec<FFProbeStream>>,
    chapters: Option<Vec<FFProbeChapter>>,
}

// Fields are individually optional: a missing key must not fail the whole
// deserialization, otherwise the video silently disappears from the library.
#[derive(Deserialize)]
struct FFProbeFormat {
    #[serde(default)]
    duration: String,
    #[serde(default)]
    size: String,
}

#[derive(Deserialize)]
struct FFProbeStream {
    #[serde(default)]
    codec_type: String,
    #[serde(default)]
    codec_name: String,
    width: Option<u32>,
    height: Option<u32>,
    #[serde(default)]
    avg_frame_rate: String,
}

#[derive(Deserialize)]
struct FFProbeChapter {
    start_time: String,
    end_time: String,
    tags: Option<FFProbeTags>,
}

#[derive(Deserialize)]
struct FFProbeTags {
    title: Option<String>,
    #[serde(rename = "TITLE")]
    title_upper: Option<String>,
}

pub async fn get_extended_metadata(path: &Path) -> Result<VideoMetadata> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-print_format", "json",
            "-show_entries", "format=duration,size,bit_rate",
            "-show_entries", "stream=codec_type,codec_name,width,height,avg_frame_rate",
            "-show_chapters",
        ])
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .context("Failed to execute ffprobe")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("ffprobe failed: {stderr}");
    }

    let raw: FFProbeOutput = serde_json::from_slice(&output.stdout)?;

    let fmt = raw.format.unwrap_or(FFProbeFormat { duration: "0".into(), size: "0".into() });
    let duration = fmt.duration.parse::<f64>().unwrap_or(0.0);
    let size = fmt.size.parse::<u64>().unwrap_or(0);

    let (width, height, fps_str, codec, audio_codecs) = if let Some(streams) = raw.streams {
        let video = streams.iter().find(|s| s.codec_type == "video");
        let (w, h, fps_str, vc) = if let Some(s) = video {
            (s.width.unwrap_or(0), s.height.unwrap_or(0), s.avg_frame_rate.clone(), s.codec_name.clone())
        } else {
            (0, 0, "0/0".to_string(), "unknown".to_string())
        };
        let mut seen = std::collections::HashSet::new();
        let audio: Vec<String> = streams.iter()
            .filter(|s| s.codec_type == "audio")
            .map(|s| s.codec_name.to_uppercase())
            .filter(|c| seen.insert(c.clone()))
            .collect();
        (w, h, fps_str, vc, audio)
    } else {
        (0, 0, "0/0".to_string(), "unknown".to_string(), vec![])
    };

    let fps = if fps_str.contains('/') {
        let parts: Vec<&str> = fps_str.split('/').collect();
        if parts.len() == 2 {
            let num: f64 = parts[0].parse().unwrap_or(0.0);
            let den: f64 = parts[1].parse().unwrap_or(0.0);
            if den > 0.0 { num / den } else { 0.0 }
        } else {
            0.0
        }
    } else {
        fps_str.parse::<f64>().unwrap_or(0.0)
    };

    let mut chapters: Vec<Chapter> = Vec::new();
    if let Some(raw_chapters) = raw.chapters {
        for ch in raw_chapters {
            let start = ch.start_time.parse::<f64>().unwrap_or(0.0);
            let end = ch.end_time.parse::<f64>().unwrap_or(0.0);
            let title = if let Some(tags) = ch.tags {
                tags.title.or(tags.title_upper).unwrap_or_default()
            } else {
                String::new()
            };

            chapters.push(Chapter {
                start,
                end: end.max(start),
                title,
            });
        }
    }

    chapters.sort_by(|a, b| a.start.total_cmp(&b.start));

    if chapters.is_empty() && duration > 0.0 {
        chapters.push(Chapter { start: 0.0, end: duration, title: String::new() });
    }

    for i in 0..chapters.len().saturating_sub(1) {
        if chapters[i].end <= chapters[i].start {
             let next_start = chapters[i+1].start;
             chapters[i].end = (next_start - 0.1).max(chapters[i].start);
        }
    }

    Ok(VideoMetadata {
        duration,
        size,
        width,
        height,
        fps,
        codec,
        audio_codecs,
        chapters,
    })
}

async fn run_ffmpeg_thumb(path: &Path, seek_time: Option<f64>, width: u16) -> Result<Vec<u8>> {
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-v", "error"]);
    if let Some(t) = seek_time {
        cmd.args(["-ss", &format!("{t:.3}")]);
    }
    cmd.arg("-i").arg(path);
    cmd.args(["-frames:v", "1"]);
    if width > 0 {
        cmd.args(["-vf", &format!("scale={width}:-2")]);
    }
    cmd.args(["-f", "image2pipe", "-vcodec", "mjpeg", "pipe:1"]);

    let output = cmd.output().await?;

    if output.status.success() && !output.stdout.is_empty() {
        Ok(output.stdout)
    } else {
        anyhow::bail!("ffmpeg produced no output")
    }
}

/// Video codecs browsers can play natively. Must match the frontend's
/// `BROWSER_COMPAT_VIDEO` list in `assets.rs`.
const BROWSER_COMPAT_VIDEO: &[&str] = &["h264", "vp8", "vp9", "av1"];

pub fn is_browser_compatible_video(codec: &str) -> bool {
    BROWSER_COMPAT_VIDEO.contains(&codec.to_lowercase().as_str())
}

/// Hardware-accelerated H.264 encoder, detected once at first use.
#[derive(Debug, Clone)]
enum HwEncoder {
    /// macOS — no special init args needed
    VideoToolbox,
    /// Linux (AMD/Intel) — needs `-vaapi_device` before input
    Vaapi,
    /// Windows AMD
    Amf,
    /// NVIDIA (all platforms)
    Nvenc,
    /// Intel `QuickSync`
    Qsv,
    /// Software fallback
    Libx264,
}

/// Verify an encoder actually works: being listed in `ffmpeg -encoders`
/// does not guarantee the driver/hardware can encode (missing permissions
/// on /dev/dri, unsupported profile, ...).
fn test_encoder(variant: &HwEncoder) -> bool {
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.args(["-v", "error"]);
    if matches!(variant, HwEncoder::Vaapi) {
        cmd.args(["-vaapi_device", "/dev/dri/renderD128"]);
    }
    cmd.args(["-f", "lavfi", "-i", "testsrc=duration=0.1:size=320x240:rate=30"]);
    match variant {
        HwEncoder::Vaapi => {
            cmd.args(["-vf", "format=nv12,hwupload", "-c:v", "h264_vaapi"]);
        }
        HwEncoder::VideoToolbox => {
            cmd.args(["-c:v", "h264_videotoolbox"]);
        }
        HwEncoder::Amf => {
            cmd.args(["-c:v", "h264_amf"]);
        }
        HwEncoder::Nvenc => {
            cmd.args(["-c:v", "h264_nvenc"]);
        }
        HwEncoder::Qsv => {
            cmd.args(["-c:v", "h264_qsv"]);
        }
        HwEncoder::Libx264 => return true,
    }
    cmd.args(["-f", "null", "-"]);
    cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    cmd.status().is_ok_and(|s| s.success())
}

fn detect_hw_encoder() -> HwEncoder {
    // Run ffmpeg -encoders synchronously (called once, cached via OnceLock)
    let Ok(output) = std::process::Command::new("ffmpeg")
        .args(["-hide_banner", "-encoders"])
        .output()
    else {
        return HwEncoder::Libx264;
    };
    let text = String::from_utf8_lossy(&output.stdout);

    // Priority order: platform-native first, then cross-platform, then software
    let candidates: &[(&str, HwEncoder)] = &[
        ("h264_videotoolbox", HwEncoder::VideoToolbox),
        ("h264_vaapi", HwEncoder::Vaapi),
        ("h264_amf", HwEncoder::Amf),
        ("h264_nvenc", HwEncoder::Nvenc),
        ("h264_qsv", HwEncoder::Qsv),
    ];

    for (name, variant) in candidates {
        if text.contains(name) {
            // VAAPI additionally requires the render device
            if matches!(variant, HwEncoder::Vaapi) && !Path::new("/dev/dri/renderD128").exists() {
                continue;
            }
            if !test_encoder(variant) {
                eprintln!("[transcode] Encoder {name} is listed but not functional, skipping");
                continue;
            }
            eprintln!("[transcode] Using hardware encoder: {name}");
            return variant.clone();
        }
    }

    eprintln!("[transcode] No hardware encoder found, using libx264");
    HwEncoder::Libx264
}

fn hw_encoder() -> &'static HwEncoder {
    static ENCODER: OnceLock<HwEncoder> = OnceLock::new();
    ENCODER.get_or_init(detect_hw_encoder)
}

pub async fn transcode_video(path: &Path, start_time: f64, copy_video: bool) -> Result<tokio::process::Child> {
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-v", "error"]);

    // No encoder needed when the video stream is copied as-is.
    let encoder = if copy_video { None } else { Some(hw_encoder()) };

    // Pre-input args for hardware device init
    if let Some(HwEncoder::Vaapi) = encoder {
        cmd.args(["-vaapi_device", "/dev/dri/renderD128"]);
    }
    if start_time > 0.0 {
        cmd.args(["-ss", &format!("{start_time:.3}")]);
    }
    cmd.arg("-i").arg(path);
    match encoder {
        // Video is already browser-compatible, only the audio needs
        // re-encoding: stream copy avoids the expensive video encode and
        // any generation loss. With -ss the copy starts at the previous
        // keyframe, which is acceptable for chapter seeks.
        None => {
            cmd.args(["-c:v", "copy"]);
        }
        Some(HwEncoder::Vaapi) => {
            cmd.args(["-vf", "format=nv12,hwupload"]);
            cmd.args(["-c:v", "h264_vaapi", "-qp", "24"]);
        }
        Some(HwEncoder::VideoToolbox) => {
            cmd.args(["-c:v", "h264_videotoolbox", "-q:v", "65"]);
        }
        Some(HwEncoder::Amf) => {
            cmd.args(["-c:v", "h264_amf", "-quality", "speed", "-rc", "cqp", "-qp_i", "24", "-qp_p", "24"]);
        }
        Some(HwEncoder::Nvenc) => {
            cmd.args(["-c:v", "h264_nvenc", "-preset", "p4", "-cq", "24"]);
        }
        Some(HwEncoder::Qsv) => {
            cmd.args(["-c:v", "h264_qsv", "-preset", "fast", "-global_quality", "24"]);
        }
        Some(HwEncoder::Libx264) => {
            cmd.args(["-c:v", "libx264", "-preset", "fast", "-crf", "22"]);
        }
    }
    cmd.args([
        "-c:a", "aac",
        "-b:a", "192k",
        "-af", "aresample=async=1",
        "-avoid_negative_ts", "make_zero",
        "-f", "mp4",
        "-movflags", "frag_keyframe+empty_moov",
        "pipe:1",
    ]);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.stdin(Stdio::null());

    let child = cmd.spawn().context("Failed to spawn ffmpeg for transcoding")?;
    Ok(child)
}

pub async fn render_thumb(path: &Path, time: f64, width: u16) -> Result<Vec<u8>> {
    if let Ok(data) = run_ffmpeg_thumb(path, Some(time), width).await {
        return Ok(data);
    }
    // Fallback to t=0 if seeking failed
    if time > 0.0
        && let Ok(data) = run_ffmpeg_thumb(path, None, width).await
    {
        return Ok(data);
    }
    anyhow::bail!("Failed to generate thumbnail")
}
