use serde::{Deserialize, Serialize};
use std::fs::File;
mod display;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct TodoItem {
    description: String,
    complete: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TodoList {
    filename: String,
    main_list: Vec<TodoItem>,
    name: String,
}

impl TodoList {
    pub fn add(&mut self, item: String) {
        let new_item = TodoItem {
            description: item,
            complete: false,
        };
        self.main_list.push(new_item);
        self.save();
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

    pub fn new(filename: String) -> Self {
        let mut list = Self {
            filename,
            main_list: Vec::new(),
            name: String::from("Main"),
        };
        list.load();
        list
    }
}
