use crate::art::video::{extract_frames, generate_ascii_frames, generate_ascii_video};
mod art;

fn main() {
    let frame_dir_path = "out/video/raw_frames";
    let ascii_frame_path = "out/video/ascii_frames";
    let video_path = "samples/rick_roll.mp4";

    extract_frames(video_path, frame_dir_path);
    generate_ascii_frames(frame_dir_path, ascii_frame_path);
    generate_ascii_video(ascii_frame_path, video_path);
}
