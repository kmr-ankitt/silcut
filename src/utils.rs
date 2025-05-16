use std::{fs, path::PathBuf};

use regex::Regex;

use crate::ffmpeg::SilenceEvent;

// This function parses the output of ffmpeg and extracts silence events
pub fn parse_silence_events(output: &str) -> Vec<SilenceEvent> {
    let re_start = Regex::new(r"silence_start: ([\d\.]+)").unwrap();
    let re_end = Regex::new(r"silence_end: ([\d\.]+) \| silence_duration: ([\d\.]+)").unwrap();

    let mut events = Vec::new();
    let mut current_start: Option<f32> = None;

    for line in output.lines() {
        if let Some(cap) = re_start.captures(line) {
            current_start = cap[1].parse::<f32>().ok();
        }

        if let Some(cap) = re_end.captures(line) {
            if let (Some(start), Ok(end), Ok(duration)) =
                (current_start, cap[1].parse::<f32>(), cap[2].parse::<f32>())
            {
                events.push(SilenceEvent {
                    start,
                    end,
                    duration,
                });
                current_start = None;
            }
        }
    }

    events
}

// This function parses the total duration of the input file from ffmpeg's output
pub fn parse_total_duration(stderr: &str) -> Option<f32> {
    for line in stderr.lines() {
        if let Some(duration_str) = line.trim().strip_prefix("Duration: ") {
            let time_str = duration_str.split(',').next()?; // "00:07:47.62"
            let parts: Vec<&str> = time_str.trim().split(':').collect();
            if parts.len() == 3 {
                let hours: f32 = parts[0].parse().ok()?;
                let minutes: f32 = parts[1].parse().ok()?;
                let seconds: f32 = parts[2].parse().ok()?;
                return Some(hours * 3600.0 + minutes * 60.0 + seconds);
            }
        }
    }
    None
}

// Convert times in seconds format to HH:MM:SS.mmm format
pub fn format_time(secs: f32) -> String {
    let total_millis = (secs * 1000.0).round() as u64;
    let millis = total_millis % 1000;
    let total_secs = total_millis / 1000;
    let secs = total_secs % 60;
    let mins = (total_secs / 60) % 60;
    let hours = total_secs / 3600;
    format!("{:02}:{:02}:{:02}.{:03}", hours, mins, secs, millis)
}

// Cleans up temporary files in the output directory
pub fn cleanup_temp_files(out_path: std::path::PathBuf, final_output: PathBuf) {
    let final_output = match final_output.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to resolve final output path: {}", e);
            return;
        }
    };

    let entries = match fs::read_dir(&out_path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!(
                "Failed to read output directory {}: {}",
                out_path.display(),
                e
            );
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            match path.canonicalize() {
                Ok(p) if p == final_output => continue, // skip the final output
                Ok(p) => {
                    if let Err(e) = fs::remove_file(&p) {
                        eprintln!("Failed to delete {}: {}", p.display(), e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to resolve path {}: {}", path.display(), e);
                }
            }
        }
    }
}
