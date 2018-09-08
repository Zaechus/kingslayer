extern crate kinggame1d;

use std::collections::HashMap;

use kinggame1d::{cli::Cli, item::Item, room::Room};

fn main() {
    // Start Room
    let iron_sword = Box::new(Item::new(
        "iron sword",
        "There is an iron sword on the ground.",
        false,
    ));
    let mut start_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    start_room_objs.insert(iron_sword.name(), iron_sword);
    let mut start_room = Box::new(Room::new(
        "Start Room",
        "You stand at the beginning.",
        start_room_objs,
    ));

    // Next Room
    let big_red_block = Box::new(Item::new(
        "big red block",
        "It's just a big red block.",
        false,
    ));
    let mut next_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    next_room_objs.insert(big_red_block.name(), big_red_block);
    let mut next_room = Box::new(Room::new(
        "Next Room",
        "You are in the next room over.",
        next_room_objs,
    ));

    // Long Hallway
    let capsule = Box::new(Item::new("capsule", "There is a capsule here.", true));
    let mut long_hallway_objs: HashMap<String, Box<Item>> = HashMap::new();
    long_hallway_objs.insert(capsule.name(), capsule);
    let mut long_hallway = Box::new(Room::new(
        "Long Hallway",
        "You are in a long, dark hallway.",
        long_hallway_objs,
    ));

    // paths
    start_room.add_path("e", &next_room.name(), "There is a pathway to the east.");
    start_room.add_path(
        "s",
        &long_hallway.name(),
        "There is a hallway to the south.",
    );
    long_hallway.add_path("n", &start_room.name(), "There is a room to the north");
    next_room.add_path("w", &start_room.name(), "There is a pathway to the west.");

    let rooms: Vec<Box<Room>> = vec![start_room, next_room, long_hallway];

    let cli = Cli::new(rooms);
    cli.start();
}
