extern crate kinggame1d;

use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

extern crate serde;
extern crate serde_json;

use kinggame1d::{Cli, Room};

fn main() -> io::Result<()> {
    let world_file = File::open("world_data/world.json").expect("can't open file");
    let mut world_file_reader = BufReader::new(world_file);
    let mut data = String::new();
    world_file_reader
        .read_to_string(&mut data)
        .expect("Unable to read string");

    let rooms: Vec<Box<Room>> = serde_json::from_str(&data).unwrap();

    let cli = Cli::new(rooms);
    cli.start();

    Ok(())
}
