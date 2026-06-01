use ab_glyph::{Font, FontRef, Glyph, point};
use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, ImageReader, Luma};
use std::fs;
use std::sync::OnceLock;

static FONT: OnceLock<FontRef<'static>> = OnceLock::new();

struct AsciiArt {
    content: String,
}

impl AsciiArt {
    pub fn new(img_path: &str, width: u32, height: u32) -> Self {
        let content = Self::img_to_ascii_string(img_path, width, height);
        AsciiArt { content }
    }

    pub fn save_ascii_txt(&self, path: &str) {
        fs::write(path, &self.content).expect("unable to write string");
    }

    pub fn save_ascii_img(&self, path: &str) {
        let img = self.ascii_string_to_img();
        img.save(path).expect("Failed to save the image");
    }

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

    fn to_ascii(pixel: u8) -> char {
        const CHARS: [char; 9] = [' ', '.', ':', '-', '+', '*', '#', '%', '@'];
        let idx = (pixel as usize * (CHARS.len() - 1)) / 255;
        CHARS[idx]
    }
    fn img_to_ascii_string(img_path: &str, width: u32, height: u32) -> String {
        let img = Self::load_image(img_path);
        let img = img.resize(width, height, FilterType::Nearest).to_luma8();
        let mut ascii_string = String::with_capacity((width * height) as usize);
        let h = img.height();
        let w = img.width();
        for y in 0..h {
            for x in 0..w {
                let Luma([v]) = *img.get_pixel(x, y);
                let c = Self::to_ascii(v);
                ascii_string.push(c);
            }
            ascii_string.push('\n');
        }
        ascii_string
    }

    fn ascii_string_to_img(&self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let font = FONT.get_or_init(|| {
            FontRef::try_from_slice(include_bytes!("../assets/fonts/DejaVuSansMono.ttf"))
                .expect("Failed to load font")
        });
        let font_size = 16.0;
        let glyph: Glyph = font.glyph_id('c').with_scale(font_size);
        let lines: Vec<&str> = self.content.lines().collect();
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
}

fn main() {
    let img_path = "samples/5.jpg";
    let ascii_txt_out_path = "out/ascii.txt";
    let ascii_img_out_path = "out/ascii.png";
    let ascii_art = AsciiArt::new(img_path, 120, 90);
    ascii_art.save_ascii_txt(ascii_txt_out_path);
    ascii_art.save_ascii_img(ascii_img_out_path);
}
