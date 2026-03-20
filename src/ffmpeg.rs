use std::path::Path;
use std::process::Stdio;
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

#[derive(Deserialize)]
struct FFProbeFormat {
    duration: String,
    size: String,
}

#[derive(Deserialize)]
struct FFProbeStream {
    codec_type: Option<String>,
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
        .args(&[
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
        // It's okay if ffprobe fails for some files, just return error
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("ffprobe failed: {}", stderr);
    }

    let raw: FFProbeOutput = serde_json::from_slice(&output.stdout)?;
    
    let fmt = raw.format.unwrap_or(FFProbeFormat { duration: "0".into(), size: "0".into() });
    let duration = fmt.duration.parse::<f64>().unwrap_or(0.0);
    let size = fmt.size.parse::<u64>().unwrap_or(0);

    let (width, height, fps_str, codec, audio_codecs) = if let Some(streams) = raw.streams {
        let video = streams.iter().find(|s| s.codec_type.as_deref() == Some("video"));
        let (w, h, fps_str, vc) = if let Some(s) = video {
            (s.width.unwrap_or(0), s.height.unwrap_or(0), s.avg_frame_rate.clone(), s.codec_name.clone())
        } else {
            (0, 0, "0/0".to_string(), "unknown".to_string())
        };
        let mut seen = std::collections::HashSet::new();
        let audio: Vec<String> = streams.iter()
            .filter(|s| s.codec_type.as_deref() == Some("audio"))
            .map(|s| s.codec_name.to_uppercase())
            .filter(|c| seen.insert(c.clone()))
            .collect();
        (w, h, fps_str, vc, audio)
    } else {
        (0, 0, "0/0".to_string(), "unknown".to_string(), vec![])
    };

    // Calculate float FPS
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

    // Process Chapters
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

    chapters.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    if chapters.is_empty() && duration > 0.0 {
        chapters.push(Chapter { start: 0.0, end: duration, title: String::new() });
    }

    // Adjust chapter ends to not overlap start of next
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

pub async fn render_thumb(path: &Path, time: f64, width: u16) -> Result<Vec<u8>> {
    let mut cmd = Command::new("ffmpeg");
    cmd.args(&[
        "-v", "error",
        "-ss", &format!("{:.3}", time),
        "-i"
    ]);
    cmd.arg(path);
    
    cmd.args(&["-frames:v", "1"]);
    if width > 0 {
        cmd.args(&["-vf", &format!("scale={}:-2", width)]);
    }
    cmd.args(&[
        "-f", "image2pipe",
        "-vcodec", "mjpeg",
        "pipe:1"
    ]);
    
    let output = cmd.output().await?;
    
    if output.status.success() && !output.stdout.is_empty() {
        return Ok(output.stdout);
    }
    
    // Fallback to t=0 if it failed
    if time > 0.0 {
         let mut cmd = Command::new("ffmpeg");
         cmd.args(&[
            "-v", "error",
            "-i"
        ]);
        cmd.arg(path);
        
        cmd.args(&["-frames:v", "1"]);
        if width > 0 {
            cmd.args(&["-vf", &format!("scale={}:-2", width)]);
        }
        cmd.args(&[
            "-f", "image2pipe",
            "-vcodec", "mjpeg",
            "pipe:1"
        ]);
        let output = cmd.output().await?;
        if output.status.success() && !output.stdout.is_empty() {
            return Ok(output.stdout);
        }
    }

    anyhow::bail!("Failed to generate thumbnail");
}
