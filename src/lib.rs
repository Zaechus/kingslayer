use std::io;

pub use game::Game;

mod entity;
mod game;
mod lexer;
mod player;
mod tokens;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
