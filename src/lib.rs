//! Kingslayer is a text-based dungeon crawler adventure game and game engine

pub use cli::Cli;
pub use entity::{
    item::{Armor, Container, Thing, Weapon},
    Enemy, Item,
};

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
