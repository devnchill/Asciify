use image::imageops::FilterType;
use image::{DynamicImage, ImageReader, Luma};
use std::fs;

fn load_image(path: &str) -> DynamicImage {
    match ImageReader::open(path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => img,

            Err(err) => {
                panic!("Failed to decode image: {} with error: {}", path, err);
            }
        },

        Err(err) => {
            panic!("Failed to open image: {} with error: {}", path, err);
        }
    }
}

const CHARS: [char; 9] = [' ', '.', ':', '-', '+', '*', '#', '%', '@'];
fn to_ascii(pixel: u8) -> char {
    let idx = (pixel as usize * (CHARS.len() - 1)) / 255;
    CHARS[idx]
}

fn img_to_ascii(img: DynamicImage, width: u32, height: u32) -> String {
    let img = img.resize(width, height, FilterType::Nearest).to_luma8();
    let mut ascii_string = String::with_capacity((width * height) as usize);
    let h = img.height();
    let w = img.width();
    for y in 0..h {
        for x in 0..w {
            let Luma([v]) = *img.get_pixel(x, y);
            let c = to_ascii(v);
            ascii_string.push(c);
        }
        ascii_string.push('\n');
    }
    ascii_string
}

fn save_ascii_img(ascii_string: &str, path: &str) {
    fs::write(path, ascii_string).expect("unable to write string");
}

fn main() {
    let img_path = "sample/fastfetch.png";
    let ascii_out_path = "ascii.txt";
    let img = load_image(img_path);
    let ascii_string = img_to_ascii(img, 120, 90);
    save_ascii_img(&ascii_string, ascii_out_path);
}
