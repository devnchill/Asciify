use crate::art::video::{extract_frames, generate_ascii_frames, generate_ascii_video};
use std::{env, process::exit};

use crate::art::ascii::{AsciiArt, AsciiMode};
mod art;

fn get_flag_and_source() -> Result<(AsciiMode, String), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "--help" {
        println!("Welcome to help menu");
        println!("Usage: asciify <flag> <media_path>");
        println!("flags: [--img , --video ]");
        println!("media_path: path to your audio/video");
        exit(0);
    }
    if args.len() != 3 {
        eprintln!("Invalid argument length");
        eprintln!("Program only accepts 2 arguments");
        return Err("Run --help to find out how to use this program".to_string());
    }
    let mode = match args[1].as_str() {
        "--img" => AsciiMode::Image,
        "--video" => AsciiMode::Video,
        other => return Err(format!("Unknown flag: {}", other)),
    };
    Ok((mode, args[2].clone()))
}

fn main() {
    let (mode, source) = match get_flag_and_source() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    match mode {
        AsciiMode::Image => {
            let mut art = AsciiArt::new(&source, AsciiMode::Image);
            art.generate(90, 120);
            art.save_ascii_img("out/ascii.png");
        }
        AsciiMode::Video => {
            let frame_dir_path = "out/video/raw_frames";
            let ascii_frame_path = "out/video/ascii_frames";
            extract_frames(&source, frame_dir_path);
            generate_ascii_frames(frame_dir_path, ascii_frame_path);
            generate_ascii_video(ascii_frame_path, &source);
        }
    }
}
