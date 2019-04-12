//! Kingslayer is a text-based dungeon crawler adventure game and game engine

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

pub use cli::Cli;

/// The Cli type
pub mod cli;

mod entities;
mod input;
mod player;
mod types;
mod utils;
mod world;
