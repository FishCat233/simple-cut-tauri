use tauri::{path::BaseDirectory, Manager};

pub fn get_ffmpeg_path(handle: &tauri::AppHandle) -> String {
    handle
        .path()
        .resolve("/tools/ffmpeg.exe", BaseDirectory::Resource)
        .unwrap()
        .to_string_lossy()
        .to_string()
}

pub fn get_ffprobe_path(handle: &tauri::AppHandle) -> String {
    handle
        .path()
        .resolve("/tools/ffprobe.exe", BaseDirectory::Resource)
        .unwrap()
        .to_string_lossy()
        .to_string()
}

#[allow(dead_code)]
pub fn get_media_info_path(handle: &tauri::AppHandle) -> String {
    handle
        .path()
        .resolve("/libs/MediaInfo.dll", BaseDirectory::Resource)
        .unwrap()
        .to_string_lossy()
        .to_string()
}
