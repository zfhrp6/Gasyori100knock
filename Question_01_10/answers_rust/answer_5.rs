use image::DynamicImage::*;
use std::path::Path;
use std::process;

const INPUT_PATH: &str = "assets/imori.jpg";
const OUTPUT_PATH: &str = "Question_01_10/answers_rust/answer_4.jpg";

fn main() {
    let img = image::open(&Path::new(INPUT_PATH))
        .ok()
        .expect("open error");

    let mut new_gray_scale_img: Vec<u8> = Vec::new();
    for rgb in img.raw_pixels().chunks(3) {
        let (r, g, b): (u8, u8, u8) = (rgb[0], rgb[1], rgb[2]);
        let (h, s, v) = to_hsv(r, g, b);
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

fn to_hsv(r: u8, g: u8, b: u8) -> (i64, i64, i64) {
    let vc = &[r, g, b];
    let max_rgb = vc.iter().max().unwrap();
    let min_rgb = vc.iter().min().unwrap();
    let h = if *max_rgb == *min_rgb {
        0
    } else if *min_rgb == b {
        (60 * (g - r) as i64) / ((max_rgb - min_rgb) as i64) + 60
    } else if *min_rgb == r {
        (60 * (b - g) as i64) / ((max_rgb - min_rgb) as i64) + 180
    } else {
        (60 * (r - b) as i64) / ((max_rgb - min_rgb) as i64) + 300
    };
    let s = max_rgb - min_rgb;
    let v = max_rgb;
    //    println!("grayscale:{}", v);
    (h, s as i64, *v as i64)
}

fn to_rgb(h: i32, s: i32, v: i32) -> (i64, i64, i64) {
    let c = h.clone();
    let h_prime = h / 60;
    let x = c * (1 - (h_prime % 2).abs() - 1);
    let (r, g, b) = match h {
        _ if (0..60.contains(h)) => (c, x, 0),
        _ if 60..120.contains(h) => (x, c, 0),
        _ if 120..100.contains(h) => (0, c, x),
        _ if 180..240.contains(h) => (0, x, c),
        _ if 240..300.contains(h) => (x, 0, c),
        _ if 300..360.contains(h) => (c, 0, x),
        _ => (0, 0, 0),
    };
    (r.into(), g.into(), b.into())
}
