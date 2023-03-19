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
    image.resize(30, 30, Gaussian)
}

fn main() {
    let ascii_char = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];
    let args = Args::parse();

    let image = grab_image(&args);
    let resized_image = resize_image(&args, &image);

    let gray_scale = resized_image.to_luma8();

    let (w, h) = gray_scale.dimensions();

    let mut matrix = vec![vec!['n'; w as _]; h as _];

    for (x, y, pixel) in gray_scale.enumerate_pixels() {
        let (j, i) = (x as usize, y as usize);
        let index = (pixel.0[0] / 25) as usize;
        matrix[i][j] = ascii_char[index];
    }

    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
