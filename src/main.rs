use image::imageops::FilterType;
use image::{DynamicImage, ImageReader, Luma};
use std::fs;
use text_to_png::{Color, TextRenderer};

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

fn img_to_ascii_string(img: DynamicImage, width: u32, height: u32) -> String {
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

fn ascii_string_to_img(ascii_string: &str) -> Vec<u8> {
    let renderer = TextRenderer::default();
    let color = Color::new(255, 255, 255);
    let text_png = renderer.render_text_to_png_data(ascii_string, 64, color);
    match text_png {
        Ok(img) => {
            println!("size: {:#?}", img.size);
            img.data
        }
        Err(err) => panic!("Unable to render ascii on image: {}", err),
    }
}

fn save_ascii_txt(ascii_string: &str, path: &str) {
    fs::write(path, ascii_string).expect("unable to write string");
}

fn save_ascii_img(img_data: &Vec<u8>, path: &str) {
    fs::write(path, img_data).expect("unable to write ascii image");
}

fn main() {
    let img_path = "samples/5.jpg";
    let ascii_txt_out_path = "out/ascii.txt";
    let ascii_img_out_path = "out/ascii.png";
    let img = load_image(img_path);
    let ascii_string = img_to_ascii_string(img, 120, 90);

    // this seems to work
    // let ascii_img = ascii_string_to_img("######");

    // This does not works
    let ascii_img = ascii_string_to_img(&ascii_string);
    save_ascii_txt(&ascii_string, ascii_txt_out_path);
    save_ascii_img(&ascii_img, ascii_img_out_path);
}
