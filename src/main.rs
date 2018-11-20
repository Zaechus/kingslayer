// Copyright (c) 2018 Maxwell Anderson

extern crate kinggame1d;

use std::collections::HashMap;

use kinggame1d::{
    cli::Cli, items::container::Container, items::item::Item, items::obj::Obj, room::Room,
};

fn main() {
    // Start Room
    let iron_sword = Box::new(Item::new(
        "iron sword",
        "There is an iron sword on the ground.",
    ));
    let mut start_room_objs: HashMap<String, Box<Obj>> = HashMap::new();
    start_room_objs.insert(iron_sword.name(), iron_sword);
    let mut start_room = Box::new(Room::new(
        "Start Room",
        "You stand at the beginning.",
        start_room_objs,
    ));

    // Closet
    let closet_objs: HashMap<String, Box<Obj>> = HashMap::new();
    let mut closet = Box::new(Room::new(
        "Closet",
        "This isn't a very large or clean closet.",
        closet_objs,
    ));

    // Next Room
    let big_red_block = Box::new(Item::new("big red block", "It's just a big red block."));
    let mut next_room_objs: HashMap<String, Box<Obj>> = HashMap::new();
    next_room_objs.insert(big_red_block.name(), big_red_block);
    let mut next_room = Box::new(Room::new(
        "Next Room",
        "You are in the next room over.",
        next_room_objs,
    ));

    // Long Hallway
    let capsule = Box::new(Container::new("capsule", "There is a capsule here."));
    let mut long_hallway_objs: HashMap<String, Box<Obj>> = HashMap::new();
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
    start_room.add_path(
        "closet",
        &closet.name(),
        "There is a closet off to the side.",
    );
    closet.add_path(
        "exit",
        &start_room.name(),
        "The door leads back into the room.",
    );
    long_hallway.add_path("n", &start_room.name(), "There is a room to the north");
    next_room.add_path("w", &start_room.name(), "There is a pathway to the west.");

    let rooms: Vec<Box<Room>> = vec![start_room, next_room, long_hallway, closet];

    let cli = Cli::new(rooms);
    cli.start();
}
