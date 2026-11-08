use std::process::Command;

pub struct Downloader;

impl Downloader {
    pub fn download(url: &str, output_path: &str, is_audio_only: bool) -> Result<String, String> {
        let yt_dlp_binary = "yt-dlp";
        let mut cmd = Command::new(yt_dlp_binary);
        
        if is_audio_only {
            cmd.args(["-x", "--audio-format", "mp3", "-P", output_path, url]);
        } else {
            cmd.args(["-f", "bestvideo+bestaudio/best", "-P", output_path, url]);
        }

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => Err(format!("فشل تشغيل محرك التحميل: {}", e)),
        }
    }
}
