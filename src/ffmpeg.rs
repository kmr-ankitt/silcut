use std::{path::PathBuf, process::Command};

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
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
