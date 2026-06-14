use std::{fs, process::Command};

use crate::art::ascii::AsciiArt;

pub fn extract_frames(video_path: &str, output_path: &str) {
    std::fs::create_dir_all(output_path).expect("couldn't create out dir");
    let frame_pattern = format!("{output_path}/frame_%06d.png");
    let status = Command::new("ffmpeg")
        .args(["-i", video_path, &frame_pattern])
        .status()
        .unwrap();
    assert!(status.success());
}

pub fn generate_ascii_frames(frames_dir_path: &str, output_path: &str) {
    std::fs::create_dir_all(output_path).expect("couldn't create out dir");
    let mut frames: Vec<_> = std::fs::read_dir(frames_dir_path)
        .unwrap()
        .map(|f| f.unwrap().path())
        .collect();
    for frame in frames {
        let name = frame
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace("frame", "ascii_frame");
        let save_path = format!("{output_path}/{name}");
        let mut art = AsciiArt::new(frame.to_str().unwrap());
        art.generate(120, 80);
        art.save_ascii_img(&save_path);
    }
    fs::remove_dir_all(frames_dir_path).expect("couldn't remove frames_dir_path");
}

pub fn generate_ascii_video(ascii_frames_path: &str) {
    let frame_pattern = format!("{ascii_frames_path}/ascii_frame_%06d.png");

    let status = Command::new("ffmpeg")
        .args([
            "-framerate",
            "30",
            "-i",
            &frame_pattern,
            "out/video/ascii_video.mp4",
        ])
        .status()
        .unwrap();

    assert!(status.success());
    fs::remove_dir_all(ascii_frames_path).expect("couldn't remove ascii_frame_path");
}
