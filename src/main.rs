mod cli;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use walkdir::WalkDir;

fn start(input: &PathBuf, output: &PathBuf, pb: ProgressBar) -> Result<(), Box<dyn Error>> {
    WalkDir::new(input)
        .into_iter()
        .try_for_each(|entry| -> Result<(), Box<dyn Error>> {
            let entry = entry?;
            let entry = entry.path();

            let relative_path = entry.strip_prefix(input)?;
            let output_path = Path::new(output).join(relative_path);

            if entry.is_dir() {
                fs::create_dir_all(&output_path)?;
            } else {
                fs::copy(entry, &output_path)?;
            }
            pb.inc(1);
            Ok(())
        })?;

    Ok(())
}

fn count_files(path: &PathBuf) -> usize {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .count()
}

fn init_pb(total_files: usize) -> ProgressBar {
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("▰▰▱▱ "),
    );
    pb.set_message("Copy in progress...");

    pb
}

fn main() {
    let args = cli::Args::parse();

    let source = PathBuf::from(args.source);
    let output = PathBuf::from(args.target);

    if !source.exists() {
        eprintln!("No such file or directory: {}", source.display());
        std::process::exit(0);
    }

    if output.exists() {
        eprintln!("Copy target path already exists: {}", output.display());
        std::process::exit(0);
    }

    let total_files = count_files(&source);

    let pb = init_pb(total_files);

    if let Err(e) = start(&source, &output, pb) {
        eprintln!("Error during copy: {}", e);
        std::process::exit(0);
    }
}
