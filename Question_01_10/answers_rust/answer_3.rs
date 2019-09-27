use image::DynamicImage::*;
use std::convert::TryFrom;
use std::path::Path;
use std::process;

const INPUT_PATH: &str = "assets/imori.jpg";
const OUTPUT_PATH: &str = "Question_01_10/answers_rust/answer_3.jpg";

fn main() {
    let img = image::open(&Path::new(INPUT_PATH))
        .ok()
        .expect("open error");

    let mut new_gray_scale_img: Vec<u8> = Vec::new();
    for rgb in img.raw_pixels().chunks(3) {
        let (r, g, b): (u8, u8, u8) = (rgb[0], rgb[1], rgb[2]);
        let scale: i64 = From::from(gray_binalize(From::from(r), From::from(g), From::from(b)));
        new_gray_scale_img.push(u8::try_from(scale).unwrap());
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
        &Path::new(OUTPUT_PATH),
        &new_gray_scale_img,
        width,
        height,
        image::ColorType::Gray(8),
    ) {
        Err(_) => process::exit(1),
        _ => (),
    }
}

fn gray_binalize(r: f64, g: f64, b: f64) -> i64 {
    //    println!("r:{}, g:{}, b:{}", r, g, b);
    let v = if 128.0 <= (0.2126 * r + 0.7152f64 * g + 0.0722f64 * b) {
        255
    } else {
        0
    };
    //    println!("grayscale:{}", v);
    v
}
