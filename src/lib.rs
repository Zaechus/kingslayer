#[cfg(not(target_arch = "wasm32"))]
use std::io;

pub use game::Game;

mod action;
mod container;
mod direction;
mod game;
mod item;
mod tokens;

#[cfg(not(target_arch = "wasm32"))]
fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
