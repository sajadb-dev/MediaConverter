extern crate ffmpeg_next as ffmpeg;
use ffmpeg::{codec, encoder, format, log, media, Rational};
use tauri::Window;
use tauri::Emitter;

#[tauri::command]
pub fn remux(
    window: Window, 
    duration: f64,
    input_path: String, 
    output_path: String
    ) -> Result<String, String> {

    ffmpeg::init().unwrap();
    log::set_level(log::Level::Warning);

    window.emit("remux-start", ()).unwrap();

    let mut ictx = format::input(&input_path).unwrap();
    let mut octx = format::output(&output_path).unwrap();

    let mut stream_mapping = vec![0isize; ictx.nb_streams() as _];
    let mut ist_time_bases = vec![Rational(0, 1); ictx.nb_streams() as _];
    let mut ost_index = 0;
    for (ist_index, ist) in ictx.streams().enumerate() {
        let ist_medium = ist.parameters().medium();
        if ist_medium != media::Type::Audio
            && ist_medium != media::Type::Video
            && ist_medium != media::Type::Subtitle
        {
            stream_mapping[ist_index] = -1;
            continue;
        }
        stream_mapping[ist_index] = ost_index;
        ist_time_bases[ist_index] = ist.time_base();
        ost_index += 1;
        let mut ost = octx.add_stream(encoder::find(codec::Id::None)).unwrap();
        ost.set_parameters(ist.parameters());
        // We need to set codec_tag to 0 lest we run into incompatible codec tag
        // issues when muxing into a different container format. Unfortunately
        // there's no high level API to do this (yet).
        unsafe {
            (*ost.parameters().as_mut_ptr()).codec_tag = 0;
        }
    }

    octx.set_metadata(ictx.metadata().to_owned());
    octx.write_header().unwrap();

    for (stream, mut packet) in ictx.packets() {
        let time_base: f64 = stream.time_base().into();
        let time = packet.pts().unwrap_or(0) as f64 * time_base;

        let progress = (time / duration) * 100.0;
        window.emit("ffmpeg-progress", progress).unwrap();

        let ist_index = stream.index();
        let ost_index = stream_mapping[ist_index];
        if ost_index < 0 {
            continue;
        }
        let ost = octx.stream(ost_index as _).unwrap();
        packet.rescale_ts(ist_time_bases[ist_index], ost.time_base());
        packet.set_position(-1);
        packet.set_stream(ost_index as _);
        packet.write_interleaved(&mut octx).unwrap();
    }

    octx.write_trailer().unwrap();

    window.emit("remux-complete", ()).unwrap();

    Ok("success".to_string())

}