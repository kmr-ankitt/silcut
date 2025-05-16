use std::path::{self, PathBuf};

use crate::{cli, ffmpeg::detect_silence};
use clap::Parser;

// Cli arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // Input file path
    #[arg(short = 'i', long)]
    file_path: path::PathBuf,

    // Output file path, if not passed then places the output file in current location
    #[arg(short = 'o', long, default_value = ".")]
    out_path: Option<PathBuf>,

    // How quiet it have to be to starting parsing
    // it can be negative value, for example -30
    #[arg( short = 's', long, default_value_t = -30, allow_hyphen_values = true)]
    pub silence: i32,

    // How much long duration of silence to be considered
    #[arg(short = 'd', long, default_value_t = 0.5, allow_hyphen_values = true)]
    pub minimum_silence_duration: f32,
}

pub fn start_silcut() {
    let args = Args::parse();

    let file_path: PathBuf = args.file_path;
    let out_path = args.out_path.unwrap_or_else(|| ".".into());
    let silence = args.silence;
    let minimum_silence_duration = args.minimum_silence_duration;

    detect_silence(file_path, out_path, silence, minimum_silence_duration);
}
