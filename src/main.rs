mod parser;

use clap::Parser;
use image::imageops::Gaussian;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use parser::Args;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn grab_image(args: &Args) -> DynamicImage {
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

fn write_output(name: &str, s: &[u8]) {
    let mut file = File::create(format!("{}.txt", name)).unwrap_or_else(|_| {
        println!("Failed to create file");
        process::exit(1);
    });

    file.write_all(s).unwrap_or_else(|_| {
        println!("Failed to write to file");
        process::exit(1);
    });
}

fn main() {
    let ascii_char = ["@", "#", "S", "%", "?", "*", "+", ";", ":", ",", "."];
    let args = Args::parse();

    let mut image = grab_image(&args);

    if args.scale != 1.0 {
        image = scale_image(&args, &image);
    } else if args.width != 0 && args.height != 0 {
        image = resize_image(&args, &image);
    }

    let gray_scale = image.to_luma8();

    let (w, h) = gray_scale.dimensions();

    let mut matrix = vec![vec!["n"; w as _]; h as _];

    for (x, y, pixel) in gray_scale.enumerate_pixels() {
        let (j, i) = (x as usize, y as usize);
        let index = (pixel.0[0] / 25) as usize;
        matrix[i][j] = ascii_char[index];
    }

    let mut string_row = vec![];

    for row in matrix.iter() {
        let string = row.join(" ");
        string_row.push(string);
    }

    let output = string_row.join("\n");
    let bytes = output.as_bytes();
    write_output(&args.output, bytes);

    println!("Wrote to file: {}.txt", args.output);
}
