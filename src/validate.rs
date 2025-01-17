//TODO: rename module
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

pub fn validate(args: Vec<String>) -> (PathBuf, PathBuf, String) {
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <source_directory> <destination_directory> <db_name>",
            args[0]
        );
        exit(1);
    }

    let source_dir = Path::new(&args[1]);
    if !source_dir.exists() || !source_dir.is_dir() {
        eprintln!("Source directory does not exist: {:?}", source_dir);
        exit(1);
    }
    let source_dir = source_dir
        .canonicalize()
        .expect("Failed to get absolute path of source directory.");
    let destination_dir = Path::new(&args[2]);
    if !destination_dir.exists() || !destination_dir.is_dir() {
        eprintln!(
            "Destination directory does not exist: {:?}",
            destination_dir
        );
        exit(1);
    }
    let destination_dir = destination_dir
        .canonicalize()
        .expect("Failed to get absolute path of destination directory.");
    let db_name = &args[3];

    return (source_dir, destination_dir, db_name.clone());
}
