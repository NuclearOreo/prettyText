mod parser;

use clap::Parser;
use image::imageops::Gaussian;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use parser::Args;

fn grab_image(args: &Args) -> DynamicImage {
    ImageReader::open(&args.file)
        .expect("Failed to read in image")
        .decode()
        .expect("Failed to decode")
}

fn resize_image(args: &Args, image: &DynamicImage) -> DynamicImage {
    let (w, h) = (args.width, args.height);
    image.resize(w, h, Gaussian)
}

fn scale_image(args: &Args, image: &DynamicImage) -> DynamicImage {
    let scale = args.scale;

    let (mut w, mut h) = image.to_luma8().dimensions();

    w = ((w as f32) * scale) as u32;
    h = ((h as f32) * scale) as u32;

    image.resize(w, h, Gaussian)
}

fn main() {
    let ascii_char = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];
    let args = Args::parse();

    let mut image = grab_image(&args);

    if args.scale != 1.0 {
        image = scale_image(&args, &image);
    } else if args.width != 0 && args.height != 0 {
        image = resize_image(&args, &image);
    }

    let gray_scale = image.to_luma8();

    let (w, h) = gray_scale.dimensions();

    let mut matrix = vec![vec!['n'; h as _]; w as _];

    for (x, y, pixel) in gray_scale.enumerate_pixels() {
        let (i, j) = (x as usize, y as usize);
        let index = (pixel.0[0] / 25) as usize;
        matrix[i][j] = ascii_char[index];
    }

    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
