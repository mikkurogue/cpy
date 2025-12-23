use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// A file or directory path to copy
    pub source: String,

    /// An output for the copy
    pub target: String,
}
