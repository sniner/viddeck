#[cfg(test)]
pub fn human_ts(sec: f64) -> String {
    let total = sec.max(0.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let m = ((total % 3600.0) / 60.0).floor() as u64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let s = total % 60.0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let h = (total / 3600.0).floor() as u64;
    if h > 0 {
        format!("{h:02}:{m:02}:{s:02.0}")
    } else {
        format!("{m:02}:{s:02.0}")
    }
}

#[cfg(test)]
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

pub fn generate_shell_html() -> String {
    String::from(r#"<!doctype html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>VidDeck</title>
    <link rel="stylesheet" href="/style.css">
</head>
<body>
    <div id="app"></div>
    <div id="lightbox" class="lightbox"></div>
    <script src="/script.js"></script>
</body>
</html>
"#)
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
