pub use cli::Cli;
pub use item::Item;
pub use room::Room;

/// Contains the Cli struct
pub mod cli;

/// Contains the Room struct
pub mod room;

/// Representations of objects found within the World
pub mod item;

/// Contains the read_line function
mod utils;

/// Contains the World struct
mod world;
