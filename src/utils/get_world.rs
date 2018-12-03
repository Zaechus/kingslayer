use std::fs::File;
use std::io::{BufReader, Read};

extern crate serde;
extern crate serde_json;

use cli::Cli;

pub fn get_world(path: &str) -> Cli {
    let world_file = File::open(path).expect("Unable to open file");
    let mut world_file_reader = BufReader::new(world_file);
    let mut data = String::new();
    world_file_reader
        .read_to_string(&mut data)
        .expect("Unable to read string");

    serde_json::from_str(&data).unwrap()
}
