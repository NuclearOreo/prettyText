use crate::parser::Args;
use image::imageops::Gaussian;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub fn grab_image(args: &Args) -> DynamicImage {
    let file = ImageReader::open(&args.file).unwrap_or_else(|_| {
        println!("Failed to open file");
        process::exit(1);
    });

    let decoded_file = file.decode().unwrap_or_else(|_| {
        println!("Failed to decode file");
        process::exit(1);
    });

    decoded_file
}

pub fn resize_image(args: &Args, image: &DynamicImage) -> DynamicImage {
    let (w, h) = (args.width, args.height);
    image.resize(w, h, Gaussian)
}

pub fn scale_image(args: &Args, image: &DynamicImage) -> DynamicImage {
    let scale = args.scale;

    let (mut w, mut h) = image.to_luma8().dimensions();

    w = ((w as f32) * scale) as u32;
    h = ((h as f32) * scale) as u32;

    image.resize(w, h, Gaussian)
}

pub fn write_output(name: &str, s: &[u8]) {
    let mut file = File::create(format!("{}.txt", name)).unwrap_or_else(|_| {
        println!("Failed to create file");
        process::exit(1);
    });

    file.write_all(s).unwrap_or_else(|_| {
        println!("Failed to write to file");
        process::exit(1);
    });

    println!("Save to {}.txt", name);
}
