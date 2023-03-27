# Todos

Easily find comments tagged as TODO, FIX, WARNING, or NOTE in your codebase.

## Installation

```
brew install opendevtools/todos/todos
```

## Usage

```
todos <path> [filter]
```

## Open using $EDITOR variable

This command will display numbers for each TODO. Post the one you want to open in the input and itwill open using the `$EDITOR` environment variable. If no variable is set, it will use vim.

```
todos <path> --open
```
