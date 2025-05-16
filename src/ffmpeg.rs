use std::{path::PathBuf, process::Command};

use crate::utils::parse_silence_events;

#[derive(Debug)]
pub struct SilenceEvent{
    pub start: f32,
    pub end: f32,
    pub duration: f32,
}

pub fn detect_silence(file_path: PathBuf, silence: i32, minimum_silence_duration: f32) {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(file_path)
        .arg("-af")
        .arg(format!("silencedetect=n={}dB:d={}", silence, minimum_silence_duration))
        .arg("-f")
        .arg("null")
        .arg("-")
        .output()
        .expect("Failed to execute ffmpeg");


    if output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let events: Vec<SilenceEvent> = parse_silence_events(&stderr);

        for event in events {
            println!("Silence detected from {} to {} (duration: {})", event.start, event.end, event.duration);
        }
    } else {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
