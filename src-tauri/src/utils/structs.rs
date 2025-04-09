use serde::Serialize;

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
