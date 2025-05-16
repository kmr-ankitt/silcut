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
            if let (Some(start), Ok(end), Ok(duration)) = (
                current_start,
                cap[1].parse::<f32>(),
                cap[2].parse::<f32>(),
            ) {
                events.push(SilenceEvent { start, end, duration });
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
