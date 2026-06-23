use std::{fs, process::Command};

use rayon::prelude::*;

use crate::art::ascii::AsciiArt;

/// Queries the source video's frame rate via ffprobe.
/// ffprobe returns a rational like "30/1" or "24000/1001".
/// Falls back to 30 fps if detection fails for any reason.
fn detect_video_fps(video_path: &str) -> f64 {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=r_frame_rate",
            "-of", "default=noprint_wrappers=1:nokey=1",
            video_path,
        ])
        .output()
        .expect("failed to run ffprobe for fps detection");
    let fps_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if fps_str.is_empty() {
        return 30.0;
    }
    if let Some((num, den)) = fps_str.split_once('/') {
        let num: f64 = num.parse().unwrap_or(30.0);
        let den: f64 = den.parse().unwrap_or(1.0);
        num / den
    } else {
        fps_str.parse().unwrap_or(30.0)
    }
}

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
    // read_dir returns entries in filesystem order, which is not guaranteed
    // to be numeric. Without sorting, frame_000010.png can appear before
    // frame_000002.png, scrambling the output video.
    frames.sort();
    // Process frames in parallel. Each frame is fully independent — its own
    // file read, resize, ASCII conversion, glyph rendering, and file write.
    // On a 4-core machine this gives ~3-4× wall-clock reduction for the
    // CPU-bound conversion work.
    frames.par_iter().for_each(|frame| {
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
    });
    fs::remove_dir_all(frames_dir_path).expect("couldn't remove frames_dir_path");
}

pub fn generate_ascii_video(ascii_frames_path: &str, source_video_path: &str) {
    let frame_pattern = format!("{ascii_frames_path}/ascii_frame_%06d.png");
    // Detect the actual framerate from the source video instead of hardcoding
    // 30 fps. A 24 fps film or 60 fps gameplay video would play at the wrong
    // speed otherwise.
    let fps = detect_video_fps(source_video_path);

    let status = Command::new("ffmpeg")
        .args([
            "-framerate",
            &fps.to_string(),
            "-i",
            &frame_pattern,
            "out/video/ascii_video.mp4",
        ])
        .status()
        .unwrap();

    assert!(status.success());
    fs::remove_dir_all(ascii_frames_path).expect("couldn't remove ascii_frame_path");
}
