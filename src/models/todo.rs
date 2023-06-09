use super::todo_type::{contains_todo_type, TodoType};
use colored::*;
use nom::{
    bytes::complete::{take_till, take_till1},
    character::complete,
    IResult,
};
use std::fmt::Display;

pub struct Todo {
    pub file_path: String,
    pub line_number: (usize, usize),
    text: String,
    todo_type: TodoType,
}

fn parse_todo(input: &str, line_number: usize, file_path: String) -> IResult<&str, Todo> {
    let (input, column) = complete::multispace0(input)?;
    let (input, comment_tag) = take_till(char::is_alphabetic)(input)?;
    let (input, todo_space) = complete::multispace0(input)?;
    let (input, todo_type) = contains_todo_type(input)?;
    let (input, _) = take_till1(char::is_alphabetic)(input)?;

    Ok((
        input,
        Todo {
            file_path,
            line_number: (
                line_number + 1,
                column.len() + comment_tag.len() + todo_space.len() + 1,
            ),
            text: input.replace("-->", "").trim().to_string(),
            todo_type: todo_type.into(),
        },
    ))
}

impl From<(usize, &str, String)> for Todo {
    fn from((line_number, text, path): (usize, &str, String)) -> Self {
        parse_todo(text, line_number, path).unwrap().1
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.todo_type,
            self.text,
            format!(
                "[{}:{}:{}]",
                self.file_path, self.line_number.0, self.line_number.1
            )
            .bright_black()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_from() {
        let todo = Todo::from((0, "  // TODO: This is a todo", "test.rs".to_string()));

        assert_eq!(todo.line_number, (1, 6));
        assert_eq!(todo.text, "This is a todo");
        assert_eq!(todo.todo_type, TodoType::Todo);
    }

    #[test]
    fn test_todo_from_with_other_delimiter() {
        let todo = Todo::from((0, "  // TODO -> This is a todo", "test.rs".to_string()));

        assert_eq!(todo.line_number, (1, 6));
        assert_eq!(todo.text, "This is a todo");
        assert_eq!(todo.todo_type, TodoType::Todo);
    }

    #[test]
    fn test_todo_from_without_spacing() {
        let todo = Todo::from((0, "//TODO: This is a todo", "test.rs".to_string()));

        assert_eq!(todo.line_number, (1, 3));
        assert_eq!(todo.text, "This is a todo");
        assert_eq!(todo.todo_type, TodoType::Todo);
    }

    #[test]
    fn todo_from_removes_closing_tag() {
        let todo = Todo::from((0, "<!-- TODO: This is a todo -->", "test.rs".to_string()));

        assert_eq!(todo.line_number, (1, 6));
        assert_eq!(todo.text, "This is a todo");
        assert_eq!(todo.todo_type, TodoType::Todo);
    }
}
