use crate::art::image::AsciiArt;
mod art;

fn main() {
    const HEIGHT: u32 = 90;
    const WIDTH: u32 = 120;
    let input_path = "samples/5.jpg";
    let txt_path = "out/ascii.txt";
    let img_path = "out/ascii.png";

    let mut art = AsciiArt::new(input_path);

    art.generate(WIDTH, HEIGHT);
    art.save_ascii_txt(txt_path);
    art.save_ascii_img(img_path);
}
