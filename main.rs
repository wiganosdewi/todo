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
        println!("type 'add', 'edit' or 'remove' to change todos");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("unable to read line");

        let input = input.trim();
        let mut vec = vec![];

        for str_element in input.split_whitespace() {
            vec.push(str_element);
        }
        match vec[0] {
            "add" => {
                vec.remove(0);
                for input in vec {
                    todos_vec.push(add_todo(id, input));
                id += 1;
                }
            }
            "edit" => continue,
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
