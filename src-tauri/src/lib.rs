// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use std::env;
use commands::probe::probe_video_detail;
use commands::thumbnail::generate_thumbnail;
use commands::metadata::get_metadata;
use commands::remux::remux;

mod commands;
mod utils;

#[tauri::command]
fn get_thumbnail_image(path: String) -> Result<Vec<u8>, String> {
    let image_data = std::fs::read(path).map_err(|e| e.to_string())?;
    Ok(image_data)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![probe_video_detail,generate_thumbnail,get_thumbnail_image,get_metadata,remux])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
