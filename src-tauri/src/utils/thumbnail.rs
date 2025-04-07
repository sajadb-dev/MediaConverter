use ffmpeg_next as ffmpeg;
use std::{env, path::Path};
use image::{ImageBuffer, Rgb};
use uuid::Uuid;

pub fn generate_thumbnail(path: &String) -> Result<String, String> {
    ffmpeg::init().map_err(|e| e.to_string())?;
    let mut ictx = ffmpeg::format::input(&path).map_err(|e| e.to_string())?;
    let stream = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or("No video stream found")?;

    let stream_index = stream.index();
    let duration_seconds = stream.duration() as f64 * f64::from(stream.time_base());

    if duration_seconds > 1.0 {
        let time_base = stream.time_base();
        let midpoint_ts = ((duration_seconds / 2.0) / f64::from(time_base)) as i64;
        ictx.seek(midpoint_ts, ..midpoint_ts + 1).map_err(|e| e.to_string())?;
    }

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

    for (stream, packet) in ictx.packets() {
        if stream.index() == stream_index {
            decoder.send_packet(&packet).map_err(|e| e.to_string())?;
            let mut decoded = ffmpeg::util::frame::Video::empty();
            if decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = ffmpeg::util::frame::Video::empty();
                scaler.run(&decoded, &mut rgb_frame).map_err(|e| e.to_string())?;

                let width = rgb_frame.width();
                let height = rgb_frame.height();
                let stride = rgb_frame.stride(0) as usize;
                let data = rgb_frame.data(0);
                let mut buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width , height);

                for y in 0..height {
                    for x in 0..width {
                        let i = y as usize * stride + x as usize * 3;
                        let pixel = Rgb([data[i], data[i + 1], data[i + 2]]);
                        buffer.put_pixel(x, y, pixel);
                    }
                }

                let temp_dir = env::temp_dir();
                let file_stem = Path::new(&path) 
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("thumb"); 
                let file_name = format!("{}_{}.jpg", file_stem, Uuid::new_v4());
                let output_path = temp_dir.join(file_name);

                buffer.save(&output_path).map_err(|e| e.to_string())?;
                return Ok(output_path.to_string_lossy().to_string());
            }
        }
    }

    Err("Failed to extract thumbnail".into())
}