use std::path::PathBuf;

use walkdir::WalkDir;

fn start(
    input: &PathBuf,
    output: &PathBuf,
    pb: indicatif::ProgressBar,
) -> Result<(), Box<dyn std::error::Error>> {
    WalkDir::new(input).into_iter().for_each(|entry| {
        let entry = entry.unwrap();
        let entry = entry.path();

        let relative_path = entry.strip_prefix(input).unwrap();
        let output_path = std::path::Path::new(output).join(relative_path);

        if entry.is_dir() {
            std::fs::create_dir_all(&output_path).unwrap();
        } else {
            std::fs::copy(entry, &output_path).unwrap();
        }
        pb.inc(1);
    });

    Ok(())
}

fn count_files(path: &PathBuf) -> usize {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .count()
}

fn init_pb(total_files: usize) -> indicatif::ProgressBar {
    let pb = indicatif::ProgressBar::new(total_files as u64);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("▰▰▱▱ "),
    );
    pb.set_message("Copy in progress...");

    pb
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("A file or directory path to copy is required");

    let output = std::env::args()
        .nth(2)
        .expect("An output for the copy is required");

    let path = PathBuf::from(path);
    let output = PathBuf::from(output);

    if !path.exists() {
        eprintln!("No such file or directory: {}", path.display());
        std::process::exit(0);
    }

    if output.exists() {
        eprintln!("Copy target path already exists: {}", output.display());
        std::process::exit(0);
    }

    let total_files = count_files(&path);

    let pb = init_pb(total_files);

    if let Err(e) = start(&path, &output, pb) {
        eprintln!("Error during copy: {}", e);
        std::process::exit(0);
    }
}
