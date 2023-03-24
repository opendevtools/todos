mod models;

use colored::*;
use jwalk::WalkDir;
use models::{todo::Todo, todo_type::contains_todo_type};
use std::{collections::HashMap, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1);
    let needle = std::env::args().nth(2);

    if path.is_none() {
        println!("Usage: todo <path>");
        return Ok(());
    }

    let path = path.unwrap();
    let mut ok_files = 0;
    let mut filtered_files = 0;
    let mut todos: HashMap<String, Vec<Todo>> = HashMap::new();
    let supported_filetypes = vec!["ts", "js", "tsx", "jsx", "vue", "html"];

    for entry in WalkDir::new(&path).sort(true) {
        let entry = entry?;

        // Skip directories
        if entry.file_type().is_dir() {
            continue;
        }

        // Skip files with unsupported filetypes
        if !supported_filetypes.contains(&entry.path().extension().unwrap().to_str().unwrap()) {
            continue;
        }

        // Read the file contents
        let file_contents = fs::read_to_string(entry.path())?;

        // Skip files without TODOs
        // if !vec!["//", "<!--"].contains(&file_contents) {
        //     ok_files += 1;
        //     continue;
        // }

        for line in file_contents.lines().enumerate() {
            if contains_todo_type(line.1).is_ok() {
                if needle.is_some() && !line.1.contains(needle.as_ref().unwrap()) {
                    filtered_files += 1;
                    continue;
                }

                todos
                    .entry(entry.path().display().to_string())
                    .or_default()
                    .push(line.into());
            }
        }
    }

    match (ok_files, filtered_files) {
        (0, 0) => println!("No TODOs found"),
        (0, 1) => println!("1 file filtered"),
        (1, 0) => println!("1 file OK"),
        (1, 1) => println!("1 file OK / 1 file filtered"),
        (_, 0) => println!("{} files OK", ok_files),
        (_, 1) => println!("{} files OK / 1 file filtered", ok_files),
        (0, _) => println!("{} files filtered", filtered_files),
        (1, _) => println!("1 file OK / {} files filtered", filtered_files),
        (_, _) => println!("{} files OK / {} files filtered", ok_files, filtered_files),
    }

    println!("");

    for (filename, todos) in todos {
        for todo in todos {
            println!(
                "{} {}",
                todo,
                filename.replace(&path, "").bold().bright_black(),
            );
        }
    }

    Ok(())
}
