use ab_glyph::{Font, Glyph, point};
use std::{fs, sync::OnceLock};

use ab_glyph::FontRef;
use image::{DynamicImage, ImageReader, Luma, imageops::FilterType};

static FONT: OnceLock<FontRef<'static>> = OnceLock::new();

pub struct AsciiArt {
    src_img: DynamicImage,
    ascii_img: Option<DynamicImage>,
    ascii_string: Option<String>,
}

impl AsciiArt {
    pub fn new(img_path: &str) -> Self {
        Self {
            src_img: Self::load_image(img_path),
            ascii_img: None,
            ascii_string: None,
        }
    }

    fn load_image(img_path: &str) -> DynamicImage {
        ImageReader::open(img_path)
            .unwrap_or_else(|err| {
                panic!("Failed to open image '{}': {}", img_path, err);
            })
            .decode()
            .unwrap_or_else(|err| {
                panic!("Failed to decode image '{}': {}", img_path, err);
            })
    }

    fn brightness_of_pixel_to_ascii_char(pixel: u8) -> char {
        const CHARS: &[u8] =
            b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
        let idx = (pixel as usize * (CHARS.len() - 1)) / 255;
        char::from(CHARS[idx])
    }

    fn img_to_ascii_string(&self, width: u32, height: u32) -> String {
        let img = self
            .src_img
            .resize(width, height, FilterType::Lanczos3)
            .to_luma8();
        let mut ascii_string = String::with_capacity((width * height + height) as usize);
        let h = img.height();
        let w = img.width();
        for y in 0..h {
            for x in 0..w {
                let Luma([v]) = *img.get_pixel(x, y);
                let c = Self::brightness_of_pixel_to_ascii_char(v);
                ascii_string.push(c);
            }
            ascii_string.push('\n');
        }
        ascii_string
    }

    fn ascii_string_to_img(&mut self) {
        let ascii_string = self
            .ascii_string
            .as_deref()
            .expect("could not find ascii string");
        let font = FONT.get_or_init(|| {
            FontRef::try_from_slice(include_bytes!("../../assets/fonts/DejaVuSansMono.ttf"))
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
        self.ascii_img = Some(image::DynamicImage::ImageLuma8(canvas))
    }

    pub fn save_ascii_txt(&self, path: &str) {
        let ascii_string = self
            .ascii_string
            .as_deref()
            .expect("ascii string not found");
        fs::write(path, ascii_string).expect("unable to write string");
    }

    pub fn save_ascii_img(&self, save_path: &str) {
        let img_data = self.ascii_img.as_ref().expect("ASCII image not generated");
        img_data.save(save_path).expect("Failed to save the image");
    }

    pub fn generate(&mut self, width: u32, height: u32) {
        let ascii = self.img_to_ascii_string(width, height);

        self.ascii_string = Some(ascii);

        self.ascii_string_to_img();
    }
}
