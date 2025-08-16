use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    description: String,
    complete: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    filename: String,
    main_list: Vec<TodoItem>,
    name: String,
}

impl TodoList {
    fn add(mut self, item: String) {
        let new_item = TodoItem {
            description: item,
            complete: false,
        };
        self.main_list.push(new_item);
        self.save();
    }

    fn print_list(self) {
        for item in self.main_list {
            println!("{:#?}", item);
        }
    }

    fn save(self) {
        let serialized = serde_yml::to_string(&self.main_list).unwrap();
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)
            .expect("Failed to open file");
        serde_yml::to_writer(file, &serialized).unwrap();
    }
}

impl Default for TodoList {
    fn default() -> TodoList {
        TodoList {
            filename: String::from("todo.yaml"),
            main_list: Vec::new(),
            name: String::from("Main"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// action - can be one of list, add, remove, complete
    action: String,

    /// object - what to do act on
    object: String,
}

fn main() {
    let args = Args::parse();

    let todo_list = TodoList::default();

    match args.action.as_str() {
        "add" => todo_list.add(args.object.clone()),
        "ls" => todo_list.print_list(),
        _ => eprintln!("unknown"),
    }

    println!("{:#?}", args);
}
