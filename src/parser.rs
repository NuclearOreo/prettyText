use clap::Parser;

/// Pretty Text, CLI to convert your photo into ASCII Art
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File path of the image
    #[arg(short, long)]
    pub file: String,

    /// Style of ASCII Art
    #[arg(short, long, default_value = "Meh")]
    pub style: String,
}
