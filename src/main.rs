use std::fs;
use std::path::Path;

fn main() {
    let source_dir = "path/to/source/directory";
    let target_dir = "path/to/target/directory";

    if !Path::new(source_dir).exists() || !Path::new(target_dir).exists() {
        println!("Error: One or both directories were not found.");
        return;
    }

    let source_files = fs::read_dir(source_dir)
        .unwrap()
        .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
        .collect::<Vec<String>>();

    let missing_files: Vec<String> = source_files
        .iter()
        .filter(|file_name| {
            !Path::new(target_dir).join(file_name).exists()
        })
        .cloned()
        .collect();

    // Выведите результаты
    if missing_files.is_empty() {
        println!("All files are present in both directories.");
    } else {
        println!("Missing files:");
        for file in missing_files {
            println!(" - {}", file);
        }
    }
}