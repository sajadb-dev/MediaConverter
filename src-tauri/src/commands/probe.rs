use crate::utils::{thumbnail::generate_thumbnail, format::{format_bitrate, detect_aspect_ratio}};
use serde::Serialize;
use std::{fs, path::Path};
use ffmpeg_next as ffmpeg;

#[derive(Serialize)]
pub struct VideoInfo {
    pub duration: String,
    pub format: String,
    pub size_bytes: u64,
    pub file_path: String,
    pub file_name: String,
    pub width: u32,
    pub height: u32,
    pub bitrate_formated: String,
    pub aspect_ratio: String,
    pub frame_rate: f64,
    pub thumbnail: String,
}

#[tauri::command]
pub fn probe_video_detail(path: String) -> Result<VideoInfo, String> {
    ffmpeg::init().map_err(|e| e.to_string())?;
    let context = ffmpeg::format::input(&path).map_err(|e| e.to_string())?;
    let duration_secs = context.duration() as f64 / ffmpeg::ffi::AV_TIME_BASE as f64;

    let total_seconds = duration_secs.round() as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let duration = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    let format = Path::new(&path)
    .extension()
    .and_then(|ext| ext.to_str())
    .unwrap_or("unknown")
    .to_lowercase();

    let metadata = fs::metadata(Path::new(&path)).map_err(|e| e.to_string())?;
    let size_bytes = metadata.len();

    let path_obj = Path::new(&path);
    let file_name = path_obj.file_name().and_then(|s| s.to_str()).unwrap_or("unknown").to_string();

    let mut width = 0;
    let mut height = 0;
    let mut bitrate_formated = String::from("unknown");
    let mut aspect_ratio = String::from("unknown");
    let mut frame_rate = 0.0;

    for stream in context.streams() {
        if stream.parameters().medium() == ffmpeg::media::Type::Video {
            let codec_ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters())
                .map_err(|e| e.to_string())?;
            let decoder = codec_ctx.decoder().video().map_err(|e| e.to_string())?;

            width = decoder.width();
            height = decoder.height();
            bitrate_formated = format_bitrate(decoder.bit_rate() as u64);
            aspect_ratio = detect_aspect_ratio(width, height);

            let avg_rate = stream.avg_frame_rate();
            if avg_rate.denominator() != 0 {
                frame_rate = avg_rate.numerator() as f64 / avg_rate.denominator() as f64;
            }
            break;
        }
    }

    let thumbnail = generate_thumbnail(&path).unwrap_or_else(|e| e);

    Ok(VideoInfo {
        duration,
        format,
        size_bytes,
        file_path: path,
        file_name,
        width,
        height,
        bitrate_formated,
        aspect_ratio,
        frame_rate,
        thumbnail
    })
}
