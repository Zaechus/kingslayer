use std::io;

pub use game::Game;

mod game;
mod item;
mod tokens;

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
