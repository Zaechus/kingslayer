use std::io;

use rayon::prelude::*;

use entity::Entity;
use item::Item;

pub use game::Game;

mod entity;
mod game;
mod item;
mod lexer;
mod player;
mod room;
mod thing;
mod tokens;

fn item_index(items: &Vec<Item>, item_name: &str) -> Option<usize> {
    items
        .par_iter()
        .position_any(|item| item.name() == item_name)
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
