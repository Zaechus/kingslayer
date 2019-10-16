//! Kingslayer is a text-based dungeon crawler adventure game and game engine

pub use cli::Cli;

/// A command line interface for controlling interactions between objects in a game
pub mod cli;

mod entity;
mod input;
mod player;
mod types;
mod world;
