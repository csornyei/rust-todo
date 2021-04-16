use dirs;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path;
mod todo;

const TODO_FOLDER: &str = ".rust_todo";
const TODO_FILENAME: &str = "todos.json";

fn main() {
    let path = match get_todo_file_path() {
        Err(why) => panic!("Can't get ToDos file! {}", why),
        Ok(path) => path,
    };
    let path_display = path.display();

    let todo_file_content = match read_file_content(&path) {
        Err(why) => panic!("Can't open file {}: {}", path_display, why),
        Ok(content) => content,
    };

    let mut parsed_todos = match parse_json(&todo_file_content) {
        Err(why) => {
            println!("Can't parse {}", why);
            return;
        }
        Ok(parsed) => parsed,
    };

    let args: Vec<String> = std::env::args().collect();

    let main_command = args[1].clone();

    match main_command.as_str() {
        "show" => {
            let mut show_done = false;
            let mut show_not_done = false;
            if args.contains(&String::from("--done")) || args.contains(&String::from("-d")) {
                show_done = true;
            }
            if args.contains(&String::from("--not-done")) || args.contains(&String::from("-n")) {
                show_not_done = true;
            }
            if show_done ^ show_not_done {
                if show_done {
                    for todo in &parsed_todos {
                        if todo.is_done() {
                            println!("{}", todo);
                        }
                    }
                }
                if show_not_done {
                    for todo in &parsed_todos {
                        if !todo.is_done() {
                            println!("{}", todo);
                        }
                    }
                }
            } else {
                let mut not_done_todos: Vec<&todo::Todo> = vec![];
                let mut done_todos: Vec<&todo::Todo> = vec![];
                for todo in &parsed_todos {
                    if todo.is_done() {
                        done_todos.push(todo);
                    } else {
                        not_done_todos.push(todo);
                    }
                }
                if done_todos.len() > 0 {
                    println!("--- DONE ---");
                    for todo in &done_todos {
                        println!("{}", todo);
                    }
                }
                if not_done_todos.len() > 0 {
                    println!("--- NOT DONE ---");
                    for todo in &not_done_todos {
                        println!("{}", todo);
                    }
                }
            }
        }
        "add" => {
            let todo_text = args[2].clone();
            parsed_todos.push(todo::Todo::new(&todo_text));
            match write_to_file(&parsed_todos, &path) {
                Ok(()) => println!("ToDo added!"),
                Err(why) => println!("Can't add ToDo! {}", why),
            };
        }
        "complete" => {
            println!("Completing a ToDo")
        }
        "not-complete" => {
            println!("ToDo is not done!")
        }
        _ => println!("Not recognized command! Try something else!"),
    }
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

fn get_todo_file_path() -> Result<path::PathBuf, std::io::Error> {
    let dir_path = create_dir()?;
    let path = create_file(dir_path)?;
    Ok(path)
}

fn read_file_content(file_path: &path::PathBuf) -> Result<std::string::String, std::io::Error> {
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn write_to_file(
    content: &Vec<todo::Todo>,
    file_path: &path::PathBuf,
) -> Result<(), std::io::Error> {
    let new_todos = serde_json::to_string(content)?;
    let mut file = File::create(file_path)?;
    file.write(new_todos.as_bytes())?;
    Ok(())
}
