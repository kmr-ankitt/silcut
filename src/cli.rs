use clap::Parser;

// Cli arguments 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args{
    // Input file path
    #[arg(short = 'i' , long)]
    pub file_path : String,
    
    // Output file path, if not passed then places the output file in current location
    #[arg(short = 'o' , long, default_value = ".")]
    pub out_path : Option<String>,
    
    // How quiet it have to be to starting parsing
    #[arg(short = 's', long, default_value_t = 30)]
    pub silence : i32,
    
    // How much long duration of silence to be considered
    #[arg(short = 'd', long, default_value_t = 0.5)]
    pub minimum_silence_duration: f32,
}
