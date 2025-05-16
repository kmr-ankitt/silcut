use std::{path::PathBuf, process::Command};

use crate::utils::{parse_silence_events, parse_total_duration};

// This stores the time frames of silence in the file
#[derive(Debug)]
pub struct SilenceEvent{
    pub start: f32,
    pub end: f32,
    pub duration: f32,
}

// This stores non-silent parts of the file
pub struct KeepSegment{
    pub start: f32,
    pub end: f32
}

// It takes the file path, silence level and minimum silence duration as input and 
// checks for silence in the file and updates SilenceEvent adn KeepSegment accordingly
pub fn detect_silence(file_path: PathBuf, silence: i32, minimum_silence_duration: f32) {
    
    // FFmpeg command to detect silence
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
        let total_duration = parse_total_duration(&stderr).unwrap_or(0.0);
        let events: Vec<SilenceEvent> = parse_silence_events(&stderr);

        // Build keep segments between silence events
        let mut keep_segments = Vec::new();
        let mut last_end = 0.0;

        // If the first silence does not start at 0, keep from 0 to first silence 
        if let Some(first_event) = events.first() {
            if first_event.start > 0.0 {
            keep_segments.push(KeepSegment {
                start: 0.0,
                end: first_event.start,
            });
            }
        }

        for event in &events {
            if last_end < event.start {
            keep_segments.push(KeepSegment {
                start: last_end,
                end: event.start,
            });
            }
            last_end = event.end;
        }

        // Add final segment if needed
        if last_end < total_duration {
            keep_segments.push(KeepSegment {
            start: last_end,
            end: total_duration,
            });
        }

        // Debug print
        for event in &events {
            println!("Silence event: {} - {} (duration: {})", event.start, event.end, event.duration);
        }

        for seg in &keep_segments {
            println!("Keep segment: {} - {}", seg.start, seg.end);
        }
    } else {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
