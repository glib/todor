// use clap::{Parser, Subcommand};
use clap::Parser;
use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct TodoItem {
    description: String,
    complete: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct TodoList {
    filename: String,
    main_list: Vec<TodoItem>,
    name: String,
}

impl TodoList {
    fn add(&mut self, item: String) {
        let new_item = TodoItem {
            description: item,
            complete: false,
        };
        self.main_list.push(new_item);
        self.save();
    }

    fn print_list(&self) {
        let file = match std::fs::OpenOptions::new().read(true).open(&self.filename) {
            Ok(file) => file,
            Err(e) => { 
                println!("Error reading file {e}");
                return;
                }
        };
        match serde_yml::from_reader::<File, Vec<TodoItem>>(file) {
            Ok(todos) => {
                for item in todos {
                    println!("{:#?}", item);
                }
            }
            Err(e) => println!("Error parsing file: {e}"),
        }
    }

    fn save(&self) {
        // let serialized = serde_yml::to_string(&self.main_list).unwrap();
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)
            .expect("Failed to open file");
        serde_yml::to_writer(file, &self.main_list).unwrap();
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
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Add { object: String },
    Ls,
}

fn main() {
    let args = Args::parse();

    let mut todo_list = TodoList::default();

    match args.command {
        Commands::Add { object } => todo_list.add(object),
        Commands::Ls => todo_list.print_list(),
    }

    // println!("{:#?}", args);
}
