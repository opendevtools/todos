mod models;

use colored::*;
use jwalk::WalkDir;
use models::{todo::Todo, todo_type::contains_todo_type};
use std::{collections::HashMap, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1);

    if path.is_none() {
        println!("Usage: todo <path>");
        return Ok(());
    }

    let path = path.unwrap();
    let mut ok_files = 0;
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
        if !contains_todo_type(&file_contents) {
            ok_files += 1;
            continue;
        }

        for line in file_contents.lines().enumerate() {
            if contains_todo_type(line.1) {
                todos
                    .entry(entry.path().display().to_string())
                    .or_default()
                    .push(line.into());
            }
        }
    }

    println!(
        "{ok_files} {} {}\n",
        match ok_files {
            1 => "file",
            _ => "files",
        },
        "OK".green(),
    );

    for file in todos {
        for todo in file.1 {
            println!(
                "{} {}",
                todo,
                file.0.replace(&path, "").bold().bright_black(),
            );
        }
    }

    Ok(())
}
