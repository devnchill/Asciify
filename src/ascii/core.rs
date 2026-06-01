use image::{DynamicImage, ImageReader};

pub fn load_image(path: &str) -> DynamicImage {
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

pub fn to_ascii(pixel: u8) -> char {
    const CHARS: [char; 9] = [' ', '.', ':', '-', '+', '*', '#', '%', '@'];
    let idx = (pixel as usize * (CHARS.len() - 1)) / 255;
    CHARS[idx]
}
