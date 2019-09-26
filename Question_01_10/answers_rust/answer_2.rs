use image::DynamicImage::*;
use std::path::Path;
use std::process;
use std::convert::{TryFrom};

fn main() {
    let img = image::open(&Path::new("assets/imori.jpg"))
        .ok()
        .expect("open error");

    let mut new_gray_scale_img: Vec<u8> = Vec::new();
    for rgb in img.raw_pixels().chunks(3) {
        let (r, g, b) : (u8, u8, u8) = (rgb[0], rgb[1], rgb[2]);
        let scale : f64 = From::from(to_gray_scale(From::from(r), From::from(g), From::from(b)));
        new_gray_scale_img.push(u8::try_from(scale as i64).unwrap());
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
        &Path::new("Question_01_10/answers_rust/answer_2.jpg"),
        &new_gray_scale_img,
        width,
        height,
        image::ColorType::Gray(8),
    ) {
        Err(_) => process::exit(1),
        _ => (),
    }
}

fn to_gray_scale(r : f64, g : f64, b : f64) -> f64{
    println!("r:{}, g:{}, b:{}", r, g, b);
    let v = 0.2126 * r + 0.7152f64 * g + 0.0722f64 * b;
    println!("grayscale:{}", v);
    v
}
