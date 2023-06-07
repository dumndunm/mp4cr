mod args;
mod config;
mod ffmpeg;
mod logger;

use config::DEFAULT_CONFIG;

fn main() {
    let args = args::collect();
    let config = match &args.qual {
        Some(quality) => config::get(quality),
        None => DEFAULT_CONFIG,
    };
    ffmpeg::run(&args, &config);
}
