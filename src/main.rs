use std::io;
use std::io::Write;

extern crate kinggame1d;

use kinggame1d::cli::Cli;

fn main() {
    let mut name;
    loop {
        print!("Enter a character name: ");
        io::stdout().flush().expect("error flushing");
        name = read_line();
        if !name.is_empty() {
            println!("Welcome, {}!", name);
            break;
        }
    }
    let name = name;
    let rooms: Vec<String> = Vec::new();
    let cli = Cli::new(name, rooms);
    cli.start();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading");
    input.trim().to_owned()
}
