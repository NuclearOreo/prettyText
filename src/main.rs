mod parser;
mod util;

use clap::Parser;
use parser::Args;
use util::{grab_image, write_output};

fn main() {
    let ascii_char = [
        b"@", b"#", b"S", b"%", b"?", b"*", b"+", b";", b":", b",", b".",
    ];
    let args = Args::parse();

    let image = grab_image(&args);

    let gray_scale = image.to_luma8();
    let mut bytes = vec![];

    for (x, y, pixel) in gray_scale.enumerate_pixels() {
        let index = (pixel.0[0] / 25) as usize;
        let delimiter = if x == 0 && y != 0 { b"\n" } else { b" " };
        bytes.append(&mut ascii_char[index].to_vec());
        bytes.append(&mut delimiter.to_vec());
    }

    write_output(&args.output, &bytes);
}
