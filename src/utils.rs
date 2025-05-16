use regex::Regex;

use crate::ffmpeg::SilenceEvent;

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
