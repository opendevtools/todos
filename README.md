# Todos

Easily find comments tagged as TODO, FIX, WARNING, or NOTE in your codebase.

## Installation

```
brew install opendevtools/todos/todos
```

## Usage

```
todos find [path]
```

Path defaults to `src` if none is provided.

## Filter TODOs

If you have a long list of TODOs and want to find something in particular, use the `--filter` flag.

```
todos find [path] --filter <filter>
```

## Open using $EDITOR variable

This command will display numbers for each TODO. Post the number you want to open in the input and it will open using the `$EDITOR` environment variable. If no variable is set, it will use vim. This can be used together with the `--filter` flag.

```
todos find [path] --open
```
