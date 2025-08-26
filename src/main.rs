// use clap::{Parser, Subcommand};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;

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
        for item in &self.main_list {
            println!("{:#?}", item);
        }
    }

    fn load(&mut self) {
        let file = match std::fs::OpenOptions::new().read(true).open(&self.filename) {
            Ok(file) => file,
            Err(e) => {
                println!("Error reading file {e}");
                self.main_list = Vec::new();
                return;
            }
        };

        match serde_yml::from_reader::<File, Vec<TodoItem>>(file) {
            Ok(todos) => self.main_list = todos,
            Err(e) => {
                println!("Error parsing file: {e}");
                self.main_list = Vec::new();
            }
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

    fn new(filename: String) -> Self {
        let mut list = Self {
            filename,
            main_list: Vec::new(),
            name: String::from("Main"),
        };
        list.load();
        list
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

    let mut todo_list = TodoList::new("todo.yaml".to_string());

    match args.command {
        Commands::Add { object } => todo_list.add(object),
        Commands::Ls => todo_list.print_list(),
    }

    // println!("{:#?}", args);
}
