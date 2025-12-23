mod cli;

use std::{
    error::Error,
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use clap::Parser;
use indicatif::{
    ProgressBar,
    ProgressStyle,
};
use rand::{
    Rng,
    rng,
};
use walkdir::WalkDir;

/// Available progress bar colors
const COLORS: &[&str] = &["red", "green", "yellow", "blue", "cyan", "magenta", "white"];

fn start(input: &PathBuf, output: &PathBuf, pb: &ProgressBar) -> Result<(), Box<dyn Error>> {
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

fn init_pb(total_files: usize) -> Result<ProgressBar, Box<dyn Error>> {
    let pb = ProgressBar::new(total_files as u64);
    let color = COLORS[rand::rng().random_range(0..COLORS.len())];

    let ps = ProgressStyle::default_bar()
        .template(&format!(
            "[{{elapsed_precise}}] [{{bar:40.{}}}] {{pos:>7}}/{{len:7}} {{msg}}",
            color
        ))?
        .progress_chars("▰▰▱▱ ");

    pb.set_style(ps);
    pb.set_message("Copy in progress...");

    Ok(pb)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    let source = PathBuf::from(args.source);
    let output = PathBuf::from(args.target);

    let flush = args.flush;

    if !source.exists() {
        eprintln!("No such file or directory: {}", source.display());
        std::process::exit(0);
    }

    if output.exists() {
        eprintln!("Copy target path already exists: {}", output.display());
        std::process::exit(0);
    }

    let total_files = count_files(&source);

    let pb = init_pb(total_files)?;

    if let Err(e) = start(&source, &output, &pb) {
        eprintln!("Error during copy: {}", e);
        std::process::exit(0);
    }

    if !flush {
        pb.finish_with_message("Copy complete");
    }

    Ok(())
}
