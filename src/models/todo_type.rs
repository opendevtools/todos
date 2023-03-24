use colored::*;
use nom::{
    branch,
    bytes::complete::{tag, take_till1},
    IResult,
};
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

pub fn contains_todo_type(input: &str) -> IResult<&str, &str> {
    let (input, _) = nom::character::complete::multispace0(input)?;
    let (input, _) = take_till1(char::is_alphabetic)(input)?;

    branch::alt((tag("TODO"), tag("FIX"), tag("WARNING"), tag("NOTE")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_types() {
        // Handles JS/TS comments
        assert!(contains_todo_type("// TODO: This is a todo").is_ok());
        assert!(contains_todo_type("// FIX: This is a fix").is_ok());
        assert!(contains_todo_type("// WARNING: This is a warning").is_ok());
        assert!(contains_todo_type("// NOTE: This is a note").is_ok());

        // Handles HTML comments
        assert!(contains_todo_type("<!-- TODO: This is a todo -->").is_ok());
        assert!(contains_todo_type("<!-- FIX: This is a fix -->").is_ok());
        assert!(contains_todo_type("<!-- WARNING: This is a warning -->").is_ok());
        assert!(contains_todo_type("<!-- NOTE: This is a note -->").is_ok());
    }

    #[test]
    fn test_todo_types_without_spacing() {
        // Handles JS/TS comments
        assert!(contains_todo_type("//TODO: This is a todo").is_ok());
        assert!(contains_todo_type("//FIX: This is a fix").is_ok());
        assert!(contains_todo_type("//WARNING: This is a warning").is_ok());
        assert!(contains_todo_type("//NOTE: This is a note").is_ok());

        // Handles HTML comments
        assert!(contains_todo_type("<!--TODO: This is a todo-->").is_ok());
        assert!(contains_todo_type("<!--FIX: This is a fix-->").is_ok());
        assert!(contains_todo_type("<!--WARNING: This is a warning-->").is_ok());
        assert!(contains_todo_type("<!--NOTE: This is a note-->").is_ok());
    }
}
