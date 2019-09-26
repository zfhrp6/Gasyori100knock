use image::DynamicImage::*;
use std::path::Path;
use std::process;

fn main() {
    let img = image::open(&Path::new("assets/imori.jpg"))
        .ok()
        .expect("open error");

    let mut new_img: Vec<u8> = Vec::new();
    for rgb in img.raw_pixels().chunks(3) {
        new_img.push(rgb[2]);
        new_img.push(rgb[1]);
        new_img.push(rgb[0]);
    }

    let img = match img {
        ImageRgb8(img) => img,
        _ => {
            eprintln!("image");
            process::exit(1);
        }
    };
    let (width, height) = (img.width(), img.height());

    match image::save_buffer(
        &Path::new("Question_01_10/answers_rust/answer_1.jpg"),
        &new_img,
        width,
        height,
        image::ColorType::RGB(8),
    ) {
        Err(_) => process::exit(1),
        _ => (),
    }
}
