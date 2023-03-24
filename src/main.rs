use colored::*;
use jwalk::WalkDir;
use std::{collections::HashMap, fmt::Display, fs};

#[derive(PartialEq, Debug)]
enum TodoType {
    Todo,
    Fix,
    Warning,
    Note,
}

impl Display for TodoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoType::Todo => write!(f, "{}", " TODO ".on_bright_blue().black()),
            TodoType::Fix => write!(f, "{}", " FIX ".on_bright_red().black()),
            TodoType::Warning => write!(f, "{}", " WARNING ".on_bright_yellow().black()),
            TodoType::Note => write!(f, "{}", " NOTE ".on_bright_green().black()),
        }
    }
}

#[derive(Debug)]
struct Todo {
    line_number: (usize, usize),
    text: String,
    todo_type: TodoType,
}

impl From<(usize, &str)> for Todo {
    fn from((line_number, text): (usize, &str)) -> Self {
        let text = text.split_once(':').unwrap();

        let (column, todo_type) = match text.0 {
            text if text.contains("// ") => text.split_once("// ").unwrap(),
            text if text.contains("<!-- ") => text.split_once("<!-- ").unwrap(),
            text if text.contains("//") => text.split_once("//").unwrap(),
            text if text.contains("<!--") => text.split_once("<!--").unwrap(),
            _ => todo!("Unknown TODO format"),
        };

        let todo_type = match todo_type {
            "TODO" => TodoType::Todo,
            "FIX" => TodoType::Fix,
            "WARNING" => TodoType::Warning,
            "NOTE" => TodoType::Note,
            tt => todo!("Unknown TODO type {tt}"),
        };

        Todo {
            line_number: (line_number + 1, column.len() + 1),
            text: text.1.trim().replace("-->", ""),
            todo_type,
        }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.todo_type,
            self.text,
            format!("[{}:{}]", self.line_number.0, self.line_number.1).bright_black()
        )
    }
}

fn contains_todo_type(text: &str) -> bool {
    let valid_todo_types = vec!["TODO", "FIX", "WARNING", "NOTE"];

    for todo_type in valid_todo_types {
        if text.contains(&format!("// {}", todo_type))
            || text.contains(&format!("<!-- {}", todo_type))
            || text.contains(&format!("//{}", todo_type))
            || text.contains(&format!("<!--{}", todo_type))
        {
            return true;
        }
    }

    false
}

fn valid_filetypes() -> Vec<&'static str> {
    vec!["ts", "js", "tsx", "jsx", "vue", "html"]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1);

    if path.is_none() {
        println!("Usage: todo <path>");
        return Ok(());
    }

    let path = path.unwrap();
    let mut ok_files = 0;
    let mut todos: HashMap<String, Vec<Todo>> = HashMap::new();

    for entry in WalkDir::new(&path).sort(true) {
        let entry = entry?;

        // Skip directories
        if entry.file_type().is_dir() {
            continue;
        }

        // Skip files with unsupported filetypes
        if !valid_filetypes().contains(&entry.path().extension().unwrap().to_str().unwrap()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_types() {
        // Handles JS/TS comments
        assert!(contains_todo_type("// TODO: This is a todo"));
        assert!(contains_todo_type("// FIX: This is a fix"));
        assert!(contains_todo_type("// WARNING: This is a warning"));
        assert!(contains_todo_type("// NOTE: This is a note"));

        // Handles HTML comments
        assert!(contains_todo_type("<!-- TODO: This is a todo -->"));
        assert!(contains_todo_type("<!-- FIX: This is a fix -->"));
        assert!(contains_todo_type("<!-- WARNING: This is a warning -->"));
        assert!(contains_todo_type("<!-- NOTE: This is a note -->"));
    }

    #[test]
    fn test_todo_types_without_spacing() {
        // Handles JS/TS comments
        assert!(contains_todo_type("//TODO: This is a todo"));
        assert!(contains_todo_type("//FIX: This is a fix"));
        assert!(contains_todo_type("//WARNING: This is a warning"));
        assert!(contains_todo_type("//NOTE: This is a note"));

        // Handles HTML comments
        assert!(contains_todo_type("<!--TODO: This is a todo-->"));
        assert!(contains_todo_type("<!--FIX: This is a fix-->"));
        assert!(contains_todo_type("<!--WARNING: This is a warning-->"));
        assert!(contains_todo_type("<!--NOTE: This is a note-->"));
    }

    #[test]
    fn test_todo_from() {
        let todo = Todo::from((0, "  // TODO: This is a todo"));

        assert_eq!(todo.line_number, (1, 3));
        assert_eq!(todo.text, "This is a todo");
        assert_eq!(todo.todo_type, TodoType::Todo);
    }

    #[test]
    fn test_todo_from_without_spacing() {
        let todo = Todo::from((0, "//TODO: This is a todo"));

        assert_eq!(todo.line_number, (1, 1));
        assert_eq!(todo.text, "This is a todo");
        assert_eq!(todo.todo_type, TodoType::Todo);
    }

    #[test]
    fn test_todo_display() {
        let todo = Todo::from((0, "  // TODO: This is a todo"));

        assert_eq!(
            format!("{}", todo),
            "\u{1b}[104;30m TODO \u{1b}[0m This is a todo \u{1b}[90m[1:3]\u{1b}[0m".to_string()
        );
    }
}
