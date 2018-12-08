use std::fs::File;
use std::io::{BufReader, Read};

use crate::cli::Cli;

/// Creates a Cli from the given file
///
/// A game should be setup as such:
/// ```
/// use kinggame1d::get_world;
///
/// fn main() {
///     let cli = get_world("data/world.json");
///     // `cli.start()` should be run here
/// }
/// ```
/// The string parameter should link to an existing file with
/// the proper JSON setup for creating a working game
pub fn get_world(path: &str) -> Cli {
    let world_file = File::open(path).expect("Unable to open file");
    let mut world_file_reader = BufReader::new(world_file);
    let mut data = String::new();
    world_file_reader
        .read_to_string(&mut data)
        .expect("Unable to read string");

    serde_json::from_str(&data).expect("Error when creating world from file.")
}
