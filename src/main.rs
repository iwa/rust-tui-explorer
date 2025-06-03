use std::io;
use std::fs::{self};
use std::path::Path;

fn main() {
    let path = ".";

    println!("--- Calculating file sizes in directory: {} ---", path);

    let mut size: i64 = 0;

    match calculate_directory_size(Path::new(path), &mut size) {
        Ok(()) => println!("\nTotal size of files: {}", format_size(size.clone())),
        Err(e) => eprintln!("\nError reading directory: {}", e),
    };
}

fn calculate_directory_size(dir: &Path, size: &mut i64) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                calculate_directory_size(&path, size)?;
            } else if path.is_file() {
                let metadata = fs::metadata(&path)?;
                let file_size = metadata.len() as i64;
                *size += file_size;
                println!("File: {:?}, Size: {} bytes", path, file_size);
            }
        }
    }

    Ok(())
}

fn format_size(size: i64) -> String {
    if size < 1024 {
        format!("{} bytes", size)
    } else if size < 1024 * 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
