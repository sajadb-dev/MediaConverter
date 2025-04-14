extern crate ffmpeg_next as ffmpeg;
use std::path::Path;
use image::{ImageBuffer, Rgb};
use uuid::Uuid;
use ffmpeg::util::frame::video::Video as Frame;

#[tauri::command]
pub fn generate_thumbnail(path: String) -> Result<String, String> {
    ffmpeg::init().map_err(|e| e.to_string())?;

    let mut ictx = ffmpeg::format::input(&path).map_err(|e| e.to_string())?;
    
    let stream = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or("No video stream found")?;

    let stream_index = stream.index();
    let duration_seconds = stream.duration() as f64 * f64::from(stream.time_base());

    let seek_time = duration_seconds / 2.0;

    let seek_ts = (seek_time / f64::from(stream.time_base())) as i64;
    ictx.seek(seek_ts, ..seek_ts + 1).map_err(|e| e.to_string())?;

    let decoder_ctx = ffmpeg::codec::context::Context::from_parameters(
        ictx.streams().nth(stream_index).unwrap().parameters(),
    )
    .map_err(|e| e.to_string())?;

    let mut decoder = decoder_ctx.decoder().video().map_err(|e| e.to_string())?;
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

    let mut rgb_frame = Frame::empty();

    for (stream, packet) in ictx.packets() {
        if stream.index() != stream_index {
            continue;
        }

        decoder.send_packet(&packet).map_err(|e| e.to_string())?;

        let mut decoded = Frame::empty();
        if decoder.receive_frame(&mut decoded).is_ok() {
            scaler.run(&decoded, &mut rgb_frame).map_err(|e| e.to_string())?;
        }
    }

    let width = rgb_frame.width();
    let height = rgb_frame.height();
    let stride = rgb_frame.stride(0) as usize;
    let data = rgb_frame.data(0);

    let mut buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let i = y as usize * stride + x as usize * 3;
            let pixel = Rgb([data[i], data[i + 1], data[i + 2]]);
            buffer.put_pixel(x, y, pixel);
        }
    }

    // Save to temp directory
    let temp_dir = std::env::temp_dir();
    let file_stem = Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("thumb");
    let file_name = format!("{}_{}.jpg", file_stem, Uuid::new_v4());
    let output_path = temp_dir.join(file_name);

    buffer.save(&output_path).map_err(|e| e.to_string())?;
    Ok(output_path.to_string_lossy().to_string())
}