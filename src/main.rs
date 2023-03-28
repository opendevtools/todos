mod models;

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input};
use jwalk::WalkDir;
use models::{todo::Todo, todo_type::contains_todo_type};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1);
    let needle = std::env::args().nth(2);

    // Display help if no path is provided
    if path.is_none() || path == Some("--help".to_string()) {
        println!("Usage: todo <path> [filter]");
        return Ok(());
    }

    let is_open_cmd = needle == Some("--open".to_string());

    let mut files_scanned = 0;
    let mut ok_files = 0;
    let mut filtered_files = 0;
    let mut todos: Vec<Todo> = vec![];

    let path = path.unwrap();
    let supported_filetypes = vec!["ts", "js", "tsx", "jsx", "vue", "html"];
    let gitignore = fs::read_to_string(".gitignore")?;
    let gitignore = gitignore
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    for entry in WalkDir::new(&path).sort(true) {
        let entry = entry?;
        files_scanned += 1;

        let is_ignored = gitignore.contains(&entry.path().display().to_string())
            || gitignore.contains(&entry.parent_path().display().to_string());

        // Skip files in .gitignore
        if is_ignored {
            continue;
        }

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

        // Skip files without comments
        if !file_contents.contains("//") && !file_contents.contains("<!--") {
            ok_files += 1;
            continue;
        }

        // Check and parse todos
        for line in file_contents.lines().enumerate() {
            if contains_todo_type(line.1).is_ok() {
                if !is_open_cmd && needle.is_some() && !line.1.contains(needle.as_ref().unwrap()) {
                    filtered_files += 1;
                    continue;
                }

                let todo = (line.0, line.1, entry.path().display().to_string());

                todos.push(todo.into());
            }
        }
    }

    // Print the results
    todos.iter().enumerate().for_each(|(index, todo)| {
        if is_open_cmd {
            println!("{index} {todo}");
        } else {
            println!("{todo}");
        }
    });

    println!(
        "\n{} {}\n{} {} scanned {} {} OK {} {} filtered",
        "Todos:".bold(),
        todos.len().to_string().blue(),
        "Files:".bold(),
        files_scanned.to_string().yellow(),
        "/".bright_black(),
        ok_files.to_string().green(),
        "/".bright_black(),
        filtered_files.to_string().red(),
    );

    // Add ability to open the selected file using --open
    if is_open_cmd {
        let selection: usize = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Open file #")
            .interact()
            .unwrap();

        let todo = &todos[selection];

        // Open the selected file using the EDITOR environment variable
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

        match editor.as_str() {
            "vim" | "nvim" => {
                std::process::Command::new(&editor)
                    .arg(todo.file_path.clone())
                    .arg(format!(
                        "+call cursor({line}, {column})",
                        line = todo.line_number.0,
                        column = todo.line_number.1
                    ))
                    .status()
                    .unwrap();
            }
            "code" => {
                let go_to_line = format!(
                    "./{file}:{line}:{column}",
                    file = todo.file_path,
                    line = todo.line_number.0,
                    column = todo.line_number.1
                );

                std::process::Command::new(&editor)
                    .arg("--goto")
                    .arg(go_to_line)
                    .status()
                    .unwrap();
            }
            editor => todo!("Unsupported editor: {editor}"),
        }
    }

    Ok(())
}
