// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use ffmpeg_next as ffmpeg;
use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;
use image::{ImageBuffer, Rgb};

#[derive(Serialize)]
struct VideoInfo {
    duration: String,
    format: String,
    size_bytes: u64,
    file_path: String,
    file_name: String,
    width: u32,
    height: u32,
    bitrate: u64,
    aspect_ratio: String,
    frame_rate: f64,
    thumbnail: String,
}

#[tauri::command]
fn probe_video_detail(path: String) -> Result<VideoInfo, String> {
    ffmpeg::init().map_err(|e| e.to_string())?;
    
    let context = ffmpeg::format::input(&path).map_err(|e| e.to_string())?;
    let duration_secs = context.duration() as f64 / ffmpeg::ffi::AV_TIME_BASE as f64;
    
    let total_seconds = duration_secs.round() as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let duration = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    let format = context.format().name().to_string();

    // Get file size
    let metadata = fs::metadata(Path::new(&path)).map_err(|e| e.to_string())?;
    let size_bytes = metadata.len();

    // Get file name and path
    let path_obj = Path::new(&path);
    let file_name = path_obj.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut width = 0;
    let mut height = 0;
    let mut bitrate = 0;
    let mut aspect_ratio = String::from("unknown");
    let mut frame_rate = 0.0;

    for stream in context.streams() {
        if stream.parameters().medium() == ffmpeg::media::Type::Video {
            // Convert parameters to codec context
            let codec_ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters())
                .map_err(|e| e.to_string())?;

            let decoder = codec_ctx.decoder().video().map_err(|e| e.to_string())?;

            width = decoder.width();
            height = decoder.height();
            bitrate = decoder.bit_rate() as u64;

            if width > 0 && height > 0 {
                let ratio = width as f64 / height as f64;
                aspect_ratio = match (ratio * 100.0).round() as u32 {
                    177..=179 => "16:9".to_string(),
                    132..=134 => "4:3".to_string(),
                    _ => format!("{:.2}:1", ratio),
                };
            }

            let avg_rate = stream.avg_frame_rate();
            if avg_rate.denominator() != 0 {
                frame_rate = avg_rate.numerator() as f64 / avg_rate.denominator() as f64;
            }

            break;
        }
    }
    let thumbnail =generate_thumbnail(&path.clone());

    Ok(VideoInfo {
        duration,
        format,
        size_bytes,
        file_path: path,
        file_name,
        width,
        height,
        bitrate,
        aspect_ratio,
        frame_rate,
        thumbnail
    })
}

fn generate_thumbnail(path: &String) -> Result<String, String> {
    // Initialize ffmpeg library
    ffmpeg::init().map_err(|e| e.to_string())?;

    // Open the video file
    let ictx = ffmpeg::format::input(&path).map_err(|e| e.to_string())?;

    // Get the first video stream
    let stream_index = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or("No video stream found")?
        .index();

    // Create a video decoder context
    let decoder_ctx = ffmpeg::codec::context::Context::from_parameters(
        ictx.streams().get(stream_index).unwrap().parameters(),
    )
    .map_err(|e| e.to_string())?;

    // Decode the video stream
    let mut decoder = decoder_ctx.decoder().video().map_err(|e| e.to_string())?;

    // Setup scaling context to RGB format for thumbnail generation
    let mut scaler = ffmpeg::software::scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        ffmpeg::software::scaling::flag::Flags::BILINEAR,
    )
    .map_err(|e| e.to_string())?;

    let mut frame_index = 0;

    // Iterate through video packets and decode
    for (stream, packet) in ictx.packets() {
        if stream.index() == stream_index {
            decoder.send_packet(&packet).map_err(|e| e.to_string())?;

            let mut decoded = ffmpeg::util::frame::Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                // Take the first frame
                if frame_index == 0 {
                    // Scale the frame to RGB
                    let mut rgb_frame = ffmpeg::util::frame::Video::empty();
                    scaler.run(&decoded, &mut rgb_frame).map_err(|e| e.to_string())?;

                    let width = rgb_frame.width();
                    let height = rgb_frame.height();
                    let data = rgb_frame.data(0);

                    // Create image buffer from RGB data
                    let buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(
                        width,
                        height,
                        data.to_vec(),
                    )
                    .ok_or("Failed to create image buffer")?;

                    // Create thumbnails directory if it doesn't exist
                    let thumb_dir = PathBuf::from("thumbnails");
                    if !thumb_dir.exists() {
                        fs::create_dir_all(&thumb_dir).map_err(|e| e.to_string())?;
                    }

                    // Generate thumbnail path and save the image
                    let file_name = Path::new(&path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("thumb");

                    let output_path = thumb_dir.join(format!("{}.jpg", file_name));
                    buffer.save(&output_path).map_err(|e| e.to_string())?;

                    // Return the path to the saved thumbnail
                    return Ok(output_path.to_string_lossy().into_owned());
                }

                frame_index += 1;
            }
        }
    }

    // Return an error if thumbnail couldn't be generated
    Err("Failed to extract thumbnail".into())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![probe_video_detail])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
