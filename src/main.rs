use std::sync::OnceLock;
mod ascii;
use ab_glyph::FontRef;
use ascii::img::AsciiArt;

static FONT: OnceLock<FontRef<'static>> = OnceLock::new();

fn main() {
    let img_path = "samples/5.jpg";
    let ascii_txt_out_path = "out/ascii.txt";
    let ascii_img_out_path = "out/ascii.png";
    let ascii_art = AsciiArt::new(img_path, 120, 90);
    ascii_art.save_ascii_txt(ascii_txt_out_path);
    ascii_art.save_ascii_img(ascii_img_out_path);
}
