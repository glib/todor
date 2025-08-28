mod cli;
mod todo;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    let mut todo_list = todo::TodoList::new("todo.yaml".to_string());

    match args.command {
        cli::Commands::Add { object } => todo_list.add(object),
        cli::Commands::Ls => todo_list.print_list(),
    }

    // println!("{:#?}", args);
}
