use ab_glyph::{Font, Glyph, point};
use std::{fs, sync::OnceLock};

use ab_glyph::FontRef;
use image::{DynamicImage, ImageReader, Luma, imageops};

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
        // for image this ramp is better
        // const CHARS: &[u8] = b" .:-=+*#%@";
        const CHARS: &[u8] =
            b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

        let idx = (pixel as usize * (CHARS.len() - 1)) / 255;
        char::from(CHARS[idx])
    }

    fn img_to_ascii_string(&self, width: u32, height: u32) -> String {
        // Resize then convert to luma. Order matters: resizing in color space
        // preserves chromatic information during Lanczos3 sampling; converting
        // to luma first would lose color detail before the downsample.
        let img = self
            .src_img
            .resize(width, height, imageops::FilterType::Lanczos3)
            .to_luma8();
        let capacity = width as usize * height as usize + height as usize;
        let mut ascii_string = String::with_capacity(capacity);
        // Iterate over rows/slices directly instead of calling get_pixel(x, y)
        // per pixel. get_pixel does bounds checks on every call; row iteration
        // is a zero-overhead pointer walk across the contiguous pixel buffer.
        for row in img.rows() {
            for pixel in row {
                ascii_string.push(Self::brightness_of_pixel_to_ascii_char(pixel[0]));
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
        // All lines produced by img_to_ascii_string have identical length
        // (width characters each). first() is O(1); the old .max() iterated
        // every line redundantly.
        let cols = lines.first().map_or(0, |l| l.len());
        let char_width = ab_glyph::FontRef::glyph_bounds(font, &glyph).width();
        let char_height = ab_glyph::FontRef::glyph_bounds(font, &glyph).height();
        let mut canvas = image::ImageBuffer::new(
            (cols as f32 * char_width) as u32,
            (rows as f32 * char_height) as u32,
        );
        for (y, line) in lines.iter().enumerate() {
            let pos_y = y as f32 * char_height;
            for (x, c) in line.chars().enumerate() {
                let pos_x = x as f32 * char_width;
                let glyph = font
                    .glyph_id(c)
                    .with_scale_and_position(font_size, point(pos_x, pos_y));
                if let Some(outlined) = font.outline_glyph(glyph) {
                    // outlined.draw() yields rasterizer-LOCAL coordinates
                    // (0..px_bounds.width, 0..px_bounds.height). The glyph
                    // position set by with_scale_and_position affects internal
                    // rasterizer offset but does NOT appear in x,y. Adding
                    // pos_x/pos_y places the glyph at its intended canvas spot.
                    outlined.draw(|x, y, c| {
                        let brightness = (c * 255.0) as u8;
                        canvas.put_pixel(
                            x + pos_x as u32,
                            y + pos_y as u32,
                            Luma([brightness]),
                        );
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
