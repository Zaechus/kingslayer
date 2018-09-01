use std::io;

pub mod cli;

pub mod obj;

pub mod path;

pub mod player;

pub mod room;

pub mod world;

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading");
    input.trim().to_owned()
}
