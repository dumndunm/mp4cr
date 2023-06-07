use super::args::Args;
use super::config::Config;
use std::process::{Command, Stdio};

pub fn run(args: &Args, config: &Config) {
    Command::new("ffmpeg")
        .arg("-i")
        .arg(&args.source)
        .arg("-preset")
        .arg("slow")
        .arg("-codec:a")
        .arg("libfdk_aac")
        .arg("-b:a")
        .arg("128k")
        .arg("-codec:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-b:v")
        .arg("750k")
        .arg("-minrate")
        .arg(&config.minrate)
        .arg("-maxrate")
        .arg(&config.maxrate)
        .arg("-bufsize")
        .arg(&config.bufsize)
        .arg("-vf")
        .arg(format!("scale=-1:{}", &config.scale))
        .arg(&args.output)
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .and_then(|mut process| process.wait())
        .expect("Failed to convert video file");
}
