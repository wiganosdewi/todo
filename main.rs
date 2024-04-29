use std::{io, process};

struct Todo {
    id: u32,
    description: String,
    done: bool,
}

fn main() {
    let mut todos_vec: Vec<Todo> = vec![];
    let mut id: u32 = 1;

    loop {
        if todos_vec.len() == 0 {
            println!("create your first todo!");
        } else {
            show_todos(&todos_vec);
        }
        println!("");
        println!("type 'add', 'check' or 'remove' to change todos or type 'q' to quit");
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
                "add" => {
                    if let Some(parameter) = parameter {
                        todos_vec.push(add_todo(id, parameter));
                        id += 1;
                    } else {
                        println!("enter a todo description after the 'add'");
                    }
                }
                "check" => {
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
                "remove" => continue,
                "help" => continue,
                "q" => process::exit(0),
                _ => {
                    println!("unknown command");
                    continue;
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

fn show_todos(todos_vec: &Vec<Todo>) {
    for todo in todos_vec {
        let done = match todo.done {
            true => "done",
            false => "not done" 
        };
        println!("{}. - {} - {done}", todo.id, todo.description);
    }
}
