use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// action - can be one of list, add, remove, complete
    action: String,

    /// object - what to do action to
    object: u32,
}
fn main() {
    let args = Args::parse();

    println!("{:#?}", args);
}
