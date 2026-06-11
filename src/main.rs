use ab_glyph::{Font, FontRef, Glyph, point};
use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, ImageReader, Luma};
use std::fs;
use std::sync::OnceLock;

static FONT: OnceLock<FontRef<'static>> = OnceLock::new();

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

fn ascii_string_to_img(ascii_string: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let font = FONT.get_or_init(|| {
        FontRef::try_from_slice(include_bytes!("../assets/fonts/DejaVuSansMono.ttf"))
            .expect("Failed to load font")
    });
    let font_size = 16.0;
    let glyph: Glyph = font.glyph_id('c').with_scale(font_size);
    let lines: Vec<&str> = ascii_string.lines().collect();
    let rows = lines.len();
    let cols = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let char_width = ab_glyph::FontRef::glyph_bounds(font, &glyph).width();
    let char_height = ab_glyph::FontRef::glyph_bounds(font, &glyph).height();
    let mut canvas = image::ImageBuffer::new(
        (cols as f32 * char_width) as u32,
        (rows as f32 * char_height) as u32,
    );
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos_x = x as f32 * char_width;
            let pos_y = y as f32 * char_height;
            let glyph = font
                .glyph_id(c)
                .with_scale_and_position(font_size, point(pos_x, pos_y));
            if let Some(outlined) = font.outline_glyph(glyph) {
                outlined.draw(|x, y, c| {
                    let brightness = (c * 255.0) as u8;
                    canvas.put_pixel(x + pos_x as u32, y + pos_y as u32, Luma([brightness]));
                });
            }
        }
    }
    canvas
}

fn save_ascii_txt(ascii_string: &str, path: &str) {
    fs::write(path, ascii_string).expect("unable to write string");
}

fn save_ascii_img(img_data: &ImageBuffer<Luma<u8>, Vec<u8>>, path: &str) {
    img_data.save(path).expect("Failed to save the image");
}

fn main() {
    let img_path = "samples/5.jpg";
    let ascii_txt_out_path = "out/ascii.txt";
    let ascii_img_out_path = "out/ascii.png";
    let img = load_image(img_path);
    let ascii_string = img_to_ascii_string(img, 120, 90);
    let ascii_img = ascii_string_to_img(&ascii_string);
    save_ascii_txt(&ascii_string, ascii_txt_out_path);
    save_ascii_img(&ascii_img, ascii_img_out_path);
}
