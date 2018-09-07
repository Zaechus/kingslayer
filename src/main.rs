extern crate kinggame1d;

use std::collections::HashMap;

use kinggame1d::{cli::Cli, item::Item, room::Room};

fn main() {
    // start room
    let iron_sword = Box::new(Item::new(
        "iron sword",
        "There is an iron sword on the ground.",
    ));
    let mut start_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    start_room_objs.insert(iron_sword.name(), iron_sword);
    let mut start_room = Box::new(Room::new(
        "Start Room",
        "You stand at the beginning.",
        start_room_objs,
    ));

    // next room
    let big_red_block = Box::new(Item::new("big red block", "It's just a big red block."));
    let mut next_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    next_room_objs.insert(big_red_block.name(), big_red_block);
    let mut next_room = Box::new(Room::new(
        "Next Room",
        "You are in the next room over.",
        next_room_objs,
    ));

    // paths
    start_room.add_path("e", &next_room.name(), "There is a hallway to the east.");
    next_room.add_path("w", &start_room.name(), "There is a hallway to the east.");

    let rooms: Vec<Box<Room>> = vec![start_room, next_room];

    let cli = Cli::new(rooms);
    cli.start();
}
