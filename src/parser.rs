use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value = "text")]
    pub medium: String,

    /// Name of the person to greet
    #[arg(short, long)]
    pub text: String,
}
