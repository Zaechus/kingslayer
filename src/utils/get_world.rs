use std::fs::File;
use std::io::{BufReader, Read};

use crate::cli::Cli;

/// Creates a Cli from the given file
///
/// A game should be setup and started like so:
/// ```
/// use kingslayer::get_world;
///
/// fn main() {
///     let cli = kingslayer::get_world("data/world.json");
///
///     println!("{}", cli.ask("l"));
///     loop {
///         # break;
///         match cli.ask(&cli.prompt()).as_str() {
///             ref s => {
///                 println!("{}", s);
///                 if s.contains("You died.") {
///                     break;
///                 }
///             }
///         }
///     }
/// }
/// ```
/// The string parameter should link to an existing file with
/// the proper JSON setup for creating a working game
pub fn get_world(path: &str) -> Cli {
    let world_file = File::open(path).expect("Unable to open world file");
    let mut world_file_reader = BufReader::new(world_file);
    let mut data = String::new();
    world_file_reader
        .read_to_string(&mut data)
        .expect("Unable to read string from world file");

    serde_json::from_str(&data).expect("Error when creating world from file.")
}

/// Creates a Cli from JSON already in str form
pub fn get_world_from_str(data: &str) -> Cli {
    serde_json::from_str(data).expect("Error when creating world from file.")
}
