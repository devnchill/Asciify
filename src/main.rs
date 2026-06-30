use crate::art::video::{extract_frames, generate_ascii_frames, generate_ascii_video};
use std::{env, process::exit};

use crate::art::ascii::AsciiArt;
mod art;

fn get_flag_and_source() -> Result<Vec<String>, String> {
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
    let media_format_flag = args[1].clone();
    let source = args[2].clone();
    Ok(vec![media_format_flag, source])
}

fn main() {
    let res = get_flag_and_source();
    let mut media_format_flag = String::new();
    let mut source = String::new();
    match res {
        Ok(v) => {
            media_format_flag = v[0].clone();
            source = v[1].clone();
        }
        Err(v) => {
            eprintln!("{}", v);
            exit(1);
        }
    }
    if media_format_flag == "--img" {
        let mut art = AsciiArt::new(&source);
        art.generate(90, 120);
        art.save_ascii_img("out/ascii.png");
    } else if media_format_flag == "-video" {
        let frame_dir_path = "out/video/raw_frames";
        let ascii_frame_path = "out/video/ascii_frames";
        extract_frames(&source, frame_dir_path);
        generate_ascii_frames(frame_dir_path, ascii_frame_path);
        generate_ascii_video(ascii_frame_path);
    } else {
        eprintln!("invalid flag.");
        eprintln!("Run --help to find out how to use this program");
        exit(1);
    }
}
