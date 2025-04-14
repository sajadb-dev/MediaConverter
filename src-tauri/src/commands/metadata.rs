extern crate ffmpeg_next as ffmpeg;
use serde::Serialize;
use std::{fs,path::Path};

#[derive(Serialize)]
pub struct VideoMetadata {
    width: u32,
    height: u32,
    bit_rate: usize,
    max_rate: usize,
    format: String,
    has_b_frames: bool,
    aspect_ratio: Option<(u32, u32)>,
    color_space: Option<String>,
    color_range: Option<String>,
    color_primaries: Option<String>,
    color_transfer_characteristic: Option<String>,
    chroma_location: Option<String>,
    references: i32,
    intra_dc_precision: u8,
}

#[derive(Serialize)]
pub struct AudioMetadata {
    bit_rate: usize,
    max_rate: usize,
    rate: u32,
    channels: u16,
    format: String,
    frames: usize,
    align: bool,
    channel_layout: String,
}

#[derive(Serialize)]
pub struct StreamMetadata {
    index: usize,
    duration_seconds: f64,
    frames: usize,
    medium: String,
    codec_id: String,
    video: Option<VideoMetadata>,
    audio: Option<AudioMetadata>,
}

#[derive(Serialize)]
pub struct FileMetadata {
    file_name: String,
    file_path: String,
    file_size: u64,
    duration: f64,
    streams: Vec<StreamMetadata>,
}

#[tauri::command]
pub fn get_metadata(path: String) -> Result<FileMetadata, String> {
    ffmpeg::init().unwrap();

    let mut streams_metadata = Vec::new();
    
    let path_obj = Path::new(&path);
    let file_name = path_obj.file_name().and_then(|s| s.to_str()).unwrap_or("unknown").to_string();

    let context = ffmpeg::format::input(&path).map_err(|e| e.to_string())?;
    let file_duration = context.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE);

    let metadata = fs::metadata(Path::new(&path)).map_err(|e| e.to_string())?;
    let size_bytes = metadata.len();

    for stream in context.streams() {
        let codec = ffmpeg::codec::context::Context::from_parameters(stream.parameters()).map_err(|e| e.to_string())?;
        let medium = codec.medium();
        let id = format!("{:?}", codec.id());

        let duration_seconds = stream.duration() as f64 * f64::from(stream.time_base());
        let mut video_metadata = None;
        let mut audio_metadata = None;

        match medium {
            ffmpeg::media::Type::Video => {
                if let Ok(video) = codec.decoder().video() {
                    video_metadata = Some(VideoMetadata {
                        width: video.width(),
                        height: video.height(),
                        bit_rate: video.bit_rate(),
                        max_rate: video.max_bit_rate(),
                        format: format!("{:?}", video.format()),
                        has_b_frames: video.has_b_frames(),
                        aspect_ratio: Some((video.aspect_ratio().0 as u32, video.aspect_ratio().1 as u32)),
                        color_space: Some(format!("{:?}", video.color_space())),
                        color_range: Some(format!("{:?}", video.color_range())),
                        color_primaries: Some(format!("{:?}", video.color_primaries())),
                        color_transfer_characteristic: Some(format!("{:?}", video.color_transfer_characteristic())),
                        chroma_location: Some(format!("{:?}", video.chroma_location())),
                        references: video.references() as i32,
                        intra_dc_precision: video.intra_dc_precision(),
                    });                    
                }
            }
            ffmpeg::media::Type::Audio => {
                if let Ok(audio) = codec.decoder().audio() {
                    audio_metadata = Some(AudioMetadata {
                        bit_rate: audio.bit_rate(),
                        max_rate: audio.max_bit_rate(),
                        rate: audio.rate(),
                        channels: audio.channels(),
                        format: format!("{:?}", audio.format()),
                        frames: audio.frames(),
                        align: audio.align() != 0,
                        channel_layout: format!("{:?}", audio.channel_layout()),
                    });                    
                }
            }
            _ => {}
        }

        streams_metadata.push(StreamMetadata {
            index: stream.index(),
            duration_seconds,
            frames: stream.frames().max(0) as usize,
            medium: format!("{:?}", medium),
            codec_id: id,
            video: video_metadata,
            audio: audio_metadata,
        });
    }


    Ok(FileMetadata {
        file_name: file_name,
        file_path: path,
        file_size: size_bytes,
        duration: file_duration,
        streams: streams_metadata,
    })
}
