use std::io;

pub use crate::{game::Game, player::Player, room::Room};

mod exit;
mod game;
mod item;
mod player;
mod room;
mod tokens;

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
