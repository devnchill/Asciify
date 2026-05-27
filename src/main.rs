use image::imageops::FilterType;
use image::{DynamicImage, ImageReader, Luma};
use std::fs::File;
use std::io::Write;

fn load_image(path: &str) -> DynamicImage {
    match ImageReader::open(path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => img,

            Err(err) => {
                panic!("Failed to decode image: {}", err);
            }
        },

        Err(err) => {
            panic!("Failed to open image: {}", err);
        }
    }
}

const CHARS: [char; 9] = [' ', '.', ':', '-', '+', '*', '#', '%', '@'];
fn to_ascii(pixel: u8) -> char {
    let idx = (pixel as usize * (CHARS.len() - 1)) / 255;
    CHARS[idx]
}

fn main() {
    let img = load_image("samples/fastfetch.png")
        .resize(120, 80, FilterType::Nearest)
        .to_luma8();
    let mut ascii_file = File::create("ascii.txt").expect("unable to create ascii file");
    for y in 0..img.height() {
        for x in 0..img.width() {
            let Luma([v]) = *img.get_pixel(x, y);
            let c = to_ascii(v);
            write!(ascii_file, "{c}").unwrap();
        }
        writeln!(ascii_file).unwrap();
    }
}
