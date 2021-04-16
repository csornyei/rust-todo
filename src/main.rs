use dirs;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path;
mod todo;

const TODO_FOLDER: &str = ".rust_todo";
const TODO_FILENAME: &str = "todos.json";

fn main() {
    let dir_path = match create_dir() {
        Err(why) => panic!("Can't create directory! {}", why),
        Ok(dir_path) => dir_path,
    };
    let path = match create_file(dir_path) {
        Err(why) => panic!("Can't create file! {}", why),
        Ok(path) => path,
    };
    let path_display = path.display();

    let todo_file_content = match read_file_content(&path) {
        Err(why) => panic!("Can't open file {}: {}", path_display, why),
        Ok(content) => content,
    };

    let parsed_todos = match parse_json(&todo_file_content) {
        Err(why) => {
            println!("Can't parse {}", why);
            return;
        }
        Ok(parsed) => parsed,
    };
    println!("{:?}", parsed_todos);
}

fn parse_json(input: &String) -> serde_json::Result<std::vec::Vec<todo::Todo>> {
    let todos: Vec<todo::Todo> = serde_json::from_str(input)?;

    Ok(todos)
}

fn create_dir() -> Result<path::PathBuf, std::io::Error> {
    let home = match dirs::home_dir() {
        None => panic!("Home directory not found!"),
        Some(home) => home,
    };
    let dir_path = home.join(TODO_FOLDER);
    if !dir_path.exists() {
        fs::create_dir(dir_path.clone())?;
    }
    Ok(dir_path)
}

fn create_file(dir_path: path::PathBuf) -> Result<path::PathBuf, std::io::Error> {
    let path = dir_path.join(TODO_FILENAME);
    if !path.exists() {
        let mut file = File::create(&path)?;
        file.write("[]".as_bytes())?;
    }

    Ok(path)
}

fn read_file_content(file_path: &path::PathBuf) -> Result<std::string::String, std::io::Error> {
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}
