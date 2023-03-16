use clap::Parser;

/// Pretty Text, CLI to convert your text or photo into ASCII Art
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Convert from text or photo, [options: text, photo]
    #[arg(short, long, default_value = "text")]
    pub medium: String,

    /// Input text you want convert into ASCII
    #[arg(short, long)]
    pub text: String,

    /// Font style of text
    #[arg(short, long, default_value = "Meh")]
    pub font: String,
}
