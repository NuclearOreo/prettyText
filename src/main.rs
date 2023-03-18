mod parser;

use clap::Parser;
use image::io::Reader as ImageReader;
use parser::Args;

fn main() {
    let args = Args::parse();

    let image = ImageReader::open(&args.file)
        .expect("Failed to read in image")
        .decode()
        .expect("Failed to decode")
        .to_rgb32f();

    let (h, w) = image.dimensions();

    println!("{}, {}", h, w);

    for &pixel in image.pixels() {
        println!("{:?}", pixel);
    }

    println!("{}", args.file);
}
