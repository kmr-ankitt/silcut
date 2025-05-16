use std::{fs::File, path::PathBuf, process::Command};
use std::io::Write;

use crate::utils::{cleanup_temp_files, format_time, parse_silence_events, parse_total_duration};

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
pub fn detect_silence(file_path: PathBuf, out_path: PathBuf, silence: i32, minimum_silence_duration: f32) {
    
    // FFmpeg command to detect silence
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file_path)
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

        let keep_segments: Vec<KeepSegment> = filter_segments(&events, total_duration);

        // Debug print
        for event in &events {
            println!("Silence event: {} - {} (duration: {})", event.start, event.end, event.duration);
        }

        for seg in &keep_segments {
            println!("Keep segment: {} - {}", seg.start, seg.end);
        }

        trim_silence(file_path, out_path, keep_segments);

    } else {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}


// Filters out the segments of silence from the events and returns the segments to keep
fn filter_segments(events: &[SilenceEvent], total_duration: f32) -> Vec<KeepSegment> {
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

    for event in events {
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

    keep_segments
}


// Trims all the segment where silence is detected
// and splits the input file into multiple files
// which will be merged in merge_segments function
fn trim_silence(file_path: PathBuf, out_path: PathBuf, keep_segments: Vec<KeepSegment>) {

    // Create the output directory if it doesn't exist
    if !out_path.exists() {
        if let Err(e) = std::fs::create_dir_all(&out_path) {
        eprintln!("Failed to create output directory {}: {}", out_path.display(), e);
        }
    }

    let mut concat_list_path = out_path.join("concat_list.txt");
    let mut concat_file = File::create(&concat_list_path).expect("Failed to create concat list file");

    for seg in &keep_segments {

        // Convert start and end to HH:MM:SS.mmm format
        let start = format_time(seg.start);
        let end = format_time(seg.end);


        // Get filename and file extension as this application supports both audio and video files
        let filename: std::borrow::Cow<'_, str> = file_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .replace(' ', "")
            .into();
        let extension: std::borrow::Cow<'_, str> = file_path
            .extension()
            .map(|e| e.to_string_lossy())
            .filter(|ext| ext.eq_ignore_ascii_case("mp3") || ext.eq_ignore_ascii_case("mp4"))
            .unwrap_or_else(|| std::borrow::Cow::Borrowed("mp3"));
        let output_file = format!(
            "{}/{}_{}_{}.{}",
            out_path.display(),
            filename,
            seg.start,
            seg.end,
            extension
        );

 
        // FFmpeg command to trim the each segment we want to keep
        let output = Command::new("ffmpeg")
            .arg("-ss")
            .arg(&start)
            .arg("-to")
            .arg(&end)
            .arg("-i")
            .arg(&file_path)
            .arg("-c")
            .arg("copy")
            .arg(&output_file)
            .output()
            .expect("Failed to execute trim silenced parts");

        if output.status.success() {
            println!(
                "Trimmed segment: {} - {} to {}",
                seg.start, seg.end, output_file
            );
            let abs_path = std::fs::canonicalize(&output_file).expect("Failed to get absolute path");
            writeln!(concat_file, "file '{}'", abs_path.display()).expect("Failed to write to concat list");
        } else {
            eprintln!("Failed to trim segment: {} - {}", seg.start, seg.end);
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
    merge_segments(file_path, &out_path);
}

// Merges all the segments into a single file and deletes the temporary files 
// created during the trimming process
fn merge_segments(file_path: PathBuf,out_path: &PathBuf) {
    let concat_list_path = out_path.join("concat_list.txt");
    let filename: std::borrow::Cow<'_, str> = file_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .replace(' ', "")
        .into();
    let extension: std::borrow::Cow<'_, str> = file_path
        .extension()
        .map(|e| e.to_string_lossy())
        .filter(|ext| ext.eq_ignore_ascii_case("mp3") || ext.eq_ignore_ascii_case("mp4"))
        .unwrap_or_else(|| std::borrow::Cow::Borrowed("mp3"));

    let final_output = out_path.join(format!("{}.{}", filename, extension));

    let output = Command::new("ffmpeg")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(&concat_list_path)
        .arg("-c")
        .arg("copy")
        .arg(&final_output)
        .output()
        .expect("Failed to merge segments");

    if output.status.success() {
        println!("Merged all segments successfully into {}", final_output.display());
        cleanup_temp_files(out_path.clone(), final_output);
    } else {
        eprintln!("Failed to merge segments.");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
