use colored::*;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum TodoType {
    Todo,
    Fix,
    Warning,
    Note,
}

impl From<&str> for TodoType {
    fn from(input: &str) -> Self {
        match input {
            "TODO" => TodoType::Todo,
            "FIX" => TodoType::Fix,
            "WARNING" => TodoType::Warning,
            "NOTE" => TodoType::Note,
            _ => todo!("Unknown TODO type"),
        }
    }
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

pub fn contains_todo_type(text: &str) -> bool {
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
}
