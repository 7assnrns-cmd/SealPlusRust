#![cfg_attr(not(debug_assertions), window_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn run_downloader_command(command_str: String) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command_str)
        .output();

    match output {
        Ok(res) => {
            if res.status.success() {
                Ok(String::from_utf8_lossy(&res.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&res.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_downloader_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
