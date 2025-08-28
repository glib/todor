impl crate::todo::TodoList {
    pub fn print_list(&self) {
        for item in &self.main_list {
            // println!("{:#?}", item);
            println!("{}\t{}", make_checkbox(item.complete), item.description)
        }
    }
}

fn make_checkbox(is_checked: bool) -> String {
    match is_checked {
        true => "[x]".to_string(),
        false => "[ ]".to_string(),
    }
}
