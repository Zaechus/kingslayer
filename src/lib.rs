use std::io;

pub use game::Game;

mod game;
mod lexer;
mod parser;
mod player;
mod room;
mod tokens;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
