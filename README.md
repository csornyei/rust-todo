# ToDo CLI

Simple Rust based ToDo app in the command line

At first run it create a `.rust-todo` folder and a `todos.json` file, where the ToDos are stored.

## Usage

- `show`: show saved ToDos
  - parameters:
    - `-d`, `--done`: show only done ToDos
    - `-n`, `--not-done`: show only not done ToDos
- `add <text>`: create a new ToDo with text
- `complete <id|text>`: set a ToDo based on id or text to done
- `not-complete <id|text>`: set a ToDo based on id or text to not done