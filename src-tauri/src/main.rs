#![cfg_attr(not(debug_assertions), window_subsystem = "windows")]

mod downloader;

#[tauri::command]
fn start_download(url: String, output_path: String, is_audio_only: bool) -> Result<String, String> {
    downloader::Downloader::download(&url, &output_path, is_audio_only)
}

#[tauri::command]
fn get_video_info(url: String) -> Result<String, String> {
    downloader::Downloader::get_info(&url)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_download, get_video_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
