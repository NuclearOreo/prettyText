mod parser;

use clap::Parser;
use parser::Args;

//https://texteditor.com/ascii-art/
fn main() {
    let args = Args::parse();
    let output = b"".to_vec();

    let test = r#"
    ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
    ██░███░█▀▄▄▀█▀▄▄▀█░███░█░███░
    ██░█░█░█░██░█░██░█▄▀░▀▄█▄▀░▀▄
    ██▄▀▄▀▄██▄▄███▄▄███▄█▄███▄█▄█
    ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀"#;

    let bytes = test.as_bytes();

    let mut byte = vec![];
    for (i, &u) in bytes.iter().enumerate() {
        byte.push(u);
        let char = String::from_utf8_lossy(&byte[..]);

        println!("{}", char);

        if i == 10 {
            break;
        }
    }

    let mut vec = bytes.to_vec();
    vec.append(&mut bytes.to_vec());

    match String::from_utf8(vec) {
        Ok(result) => {
            println!("{}", result);
        }
        Err(_) => panic!("Failed to output Art"),
    };
}
