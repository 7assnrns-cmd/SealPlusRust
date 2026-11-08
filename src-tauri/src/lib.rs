use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaFormat {
    pub format_id: String,
    pub ext: String,
    pub resolution: String,
    pub quality_label: String,
    pub filesize: u64,
    pub bitrate: f32,
    pub note: String,
    pub is_video: bool,
}

#[tauri::command]
pub async fn fetch_media_formats(url: String) -> Result<Vec<MediaFormat>, String> {
    let output = Command::new("yt-dlp")
        .args(["--dump-json", &url])
        .output()
        .map_err(|e| format!("فشل تشغيل محرك النواة: {}", e))?;

    if !output.status.success() {
        return Err("خطأ في جلب بيانات الرابط، تحقق من الاتصال أو صحة الرابط".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("فشل تحليل بيانات النواة: {}", e))?;

    let mut formats_list = Vec::new();

    if let Some(formats) = json["formats"].as_array() {
        for f in formats {
            let format_id = f["format_id"].as_str().unwrap_or("").to_string();
            let ext = f["ext"].as_str().unwrap_or("").to_string();
            let vcodec = f["vcodec"].as_str().unwrap_or("none");
            
            let is_video = vcodec != "none";
            let width = f["width"].as_u64().unwrap_or(0);
            let height = f["height"].as_u64().unwrap_or(0);
            let fps = f["fps"].as_f64().unwrap_or(30.0);
            let filesize = f["filesize"].as_u64().unwrap_or(f["filesize_approx"].as_u64().unwrap_or(0));
            let bitrate = f["tbr"].as_f64().unwrap_or(f["abr"].as_f64().unwrap_or(0.0)) as f32;

            let mut resolution = "Audio Only".to_string();
            let mut quality_label = format!("{} kbps", bitrate as u32);
            let mut note = "صوت نقي 🎵".to_string();

            if is_video {
                resolution = format!("{}x{}", width, height);
                quality_label = match height {
                    2160 => format!("{}p (4K 🚀)", height),
                    1440 => format!("{}p (2K ✨)", height),
                    1080 => format!("{}p (1080p {}fps)", height, fps as u32),
                    720 => format!("{}p (HD)", height),
                    _ => format!("{}p", height),
                };
                
                // تصحيح مشكلة الـ semicolon والـ String return
                note = if filesize > 0 && filesize < 20_000_000 {
                    "حجم أقل 📉".to_string()
                } else {
                    "أفضل جودة ✨".to_string()
                };
            }

            formats_list.push(MediaFormat {
                format_id,
                ext,
                resolution,
                quality_label,
                filesize,
                bitrate,
                note,
                is_video,
            });
        }
    }
    Ok(formats_list)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_media_formats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
