pub fn format_bitrate(bitrate: u64) -> String {
    if bitrate >= 1_000_000 {
        format!("{:.2} Mbps", bitrate as f64 / 1_000_000.0)
    } else if bitrate >= 1_000 {
        format!("{:.0} Kbps", bitrate as f64 / 1_000.0)
    } else {
        format!("{} bps", bitrate)
    }
}

pub fn detect_aspect_ratio(width: u32, height: u32) -> String {
    let common_ratios = [
        (16.0 / 9.0, "16:9"),
        (9.0 / 16.0, "9:16"),
        (4.0 / 3.0, "4:3"),
        (3.0 / 4.0, "3:4"),
        (1.0, "1:1"),
        (21.0 / 9.0, "21:9"),
        (18.0 / 9.0, "18:9"),
    ];

    let tolerance = 0.05;
    if height == 0 { return "unknown".to_string(); }

    let ratio = width as f64 / height as f64;
    for (val, label) in common_ratios {
        if (ratio - val).abs() < tolerance {
            return label.to_string();
        }
    }

    format!("{:.2}:1", ratio)
}