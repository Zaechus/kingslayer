use std::io;

pub use game::Game;

mod entity;
mod game;
mod lexer;
mod parse;
mod player;
mod tokens;

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
