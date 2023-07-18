use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());

    let longest_file = find_longest_file(&dir);

    match longest_file {
        Some(file) => println!("Longest file name is: {}", file),
        None => println!("No files found."),
    }
}

fn find_longest_file(dir: &PathBuf) -> Option<String> {
    let mut longest_filename: Option<String> = None;
    let mut longest_length: usize = 0;

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if let Some(file_name) = entry.file_name().to_str() {
            let file_path = entry.path();
            let file_length = file_name.chars().count();

            if file_length > longest_length {
                longest_filename = Some(file_path.to_string_lossy().into_owned());
                longest_length = file_length;
            }
        }
    }

    longest_filename
}
