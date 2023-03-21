use clap::Parser;

/// Pretty Text, CLI to convert your photo into ASCII Art
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File path of the image
    #[arg(short, long)]
    pub file: String,

    /// Style of ASCII Art
    #[arg(long, default_value = "Meh")]
    pub style: String,

    /// Resize width of image
    #[arg(long, default_value_t = 0)]
    pub width: u32,

    /// Resize height of image
    #[arg(long, default_value_t = 0)]
    pub height: u32,

    /// Resize image by scale, this preserve the aspect ratio
    #[arg(short, long, default_value_t = 1.0)]
    pub scale: f32,
}
