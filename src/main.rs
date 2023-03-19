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
    image.resize(10, 10, Gaussian)
}

fn main() {
    let _ASCII_CHAR = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];
    let args = Args::parse();

    let image = grab_image(&args);
    let resized_image = resize_image(&args, &image);

    let _gray_scale = image.to_luma8();
}
