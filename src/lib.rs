//! Kingslayer is a text-based dungeon crawler adventure game and game engine

pub use cli::Cli;
pub use entity::{
    item::{Armor, Container, Gold, Thing, Weapon},
    Element, Enemy, Item,
};
pub use types::EnemyStatus;

/// A command line interface for controlling interactions between objects in a game
mod cli;

/// Various different kinds of repetitive entities that make up a World
mod entity;

/// Methods for reading, lexing, and parsing user input
mod input;

/// A Player's inventory
mod inventory;

/// An abstraction of the player's interactions with the World
mod player;

/// Useful types used throughout the crate
mod types;

/// Manages the map of Rooms
mod world;

use rand::Rng;

fn dice_roll(num_rolls: u32, num_sides: u32) -> u32 {
    let mut sum = 0;
    for _ in 0..num_rolls {
        sum += rand::thread_rng().gen_range(0, num_sides) + 1;
    }
    sum
}
