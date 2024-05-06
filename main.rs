use std::io::{BufReader, Write, Read};
use std::{io, process};
use std::fs::{File, OpenOptions};
use std::path::Path;

struct Todo {
    id: u32,
    description: String,
    done: bool,
}

fn main() {
    let mut todos_vec: Vec<Todo> = vec![];
    let mut id: u32 = 0;

    loop {
        if todos_vec.len() == 0 {
            println!("create your first todo!");
        } else {
            let mut checked_todos = 0;
            for todo in &todos_vec {
                if todo.done {
                    checked_todos += 1;
                }
            }
            if checked_todos == todos_vec.len() {
                println!("Congratulations! You have completed all your todos!");
                println!("");
            }
            show_todos(&todos_vec);
        }
        println!("");
        println!("type [a]dd  [c]heck  [r]emove  or [q]uit to exit");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("unable to read line");

        let command: &str;
        let parameter: Option<&str>;

        match input.trim().split_once(" ") {
            Some(t) => (command, parameter) = (t.0, Some(t.1)),
            None => {
                command = input.trim();
                parameter = None;
            }
        }

        match command {
            "a" | "add" => {
                if let Some(parameter) = parameter {
                    todos_vec.push(add_todo(id, parameter));
                    id += 1;
                } else {
                    println!("enter a todo description after the 'add'");
                }
            }
            "c" | "check" => {
                if let Some(parameter) = parameter {
                    let parameter: usize = match parameter.parse() {
                        Ok(t) => t,
                        Err(e) => {
                            println!("type a number, {e}");
                            continue;
                        }
                    };
                    todos_vec[parameter].done = !todos_vec[parameter].done;
                } else {
                    println!("enter a id for the todo you want to check");
                }
            }
            "r" | "remove" => { 
                if let Some(parameter) = parameter {
                    let parameter: usize = match parameter.parse() {
                        Ok(t) => t,
                        Err(e) => {
                            println!("type a number, {e}");
                            continue;
                        }
                    };
                    todos_vec.remove(parameter);
                } else {
                    println!("enter a id for the todo you want to remove");
                }
            }
            "s" | "save" => {
                if let Some(parameter) = parameter {
                    match save_todos(&todos_vec, parameter) {
                        Ok(_t) => println!("file saved successfully"),
                        Err(e) => println!("{e}"),
                    }
                }
            }
            "l" | "load" => {
                if let Some(parameter) = parameter {
                    match read_todos(parameter) {
                        Ok(_t) => println!("file loaded successfully"),
                        Err(e) => println!("{e}"),
                    }
                }
            }
            "h" | "help" => {
                println!("list of commands");
                println!("");
                println!(" - add - type 'add' or 'a' and a todo item description to add a new todo");
                println!(" - check - type 'check' or 'c' and a todo item id to change from not done to done and the other way around");
                println!(" - remove - type 'remove' or 'r' and a todo item id to remove a todo from the list");
            }
            "q" | "quit" => process::exit(0),
            _ => {
                println!("unknown command");
            }
        }

        println!("");
    }
}

fn add_todo(id: u32, input: &str) -> Todo{
    return Todo {
        id,
        description: input.to_string(),
        done: false,
    };
}

fn save_todos(vec: &Vec<Todo>, path: &str) -> std::io::Result<()> {
    let mut file = match Path::new(path).exists() {
        true => OpenOptions::new().write(true).open(path)?,
        false => File::create(path)?,
    };
    for todo in vec {
        write!(file, "{} {} {} ", todo.id, todo.description, todo.done)?;
    }
    return Ok(());
}

fn read_todos(path: &str) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(&file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn show_todos(todos_vec: &Vec<Todo>) {
    for todo in todos_vec {
        let done = match todo.done {
            true => "done",
            false => "not done" 
        };
        println!("{}. - {} - {}", todo.id, todo.description, done);
    }
}
