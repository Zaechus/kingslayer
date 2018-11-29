use std::collections::HashMap;

use item::Item;
use room::Room;

#[test]
fn room_addpath() {
    // Start Room
    let start_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    let mut start_room = Box::new(Room::new(
        "Start Room",
        "You stand at the beginning.",
        start_room_objs,
    ));

    // Closet
    let closet_objs: HashMap<String, Box<Item>> = HashMap::new();
    let mut closet = Box::new(Room::new(
        "Closet",
        "This isn't a very large or clean closet.",
        closet_objs,
    ));

    // Next Room
    let next_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    let mut next_room = Box::new(Room::new(
        "Next Room",
        "You are in the next room over.",
        next_room_objs,
    ));

    // Long Hallway
    let long_hallway_objs: HashMap<String, Box<Item>> = HashMap::new();
    let mut long_hallway = Box::new(Room::new(
        "Long Hallway",
        "You are in a long, dark hallway.",
        long_hallway_objs,
    ));
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

    assert_eq!(
        start_room.desc(),
        format!(
            "{}\n{}\n{}\n{}\n{}",
            "Start Room",
            "You stand at the beginning.",
            "There is a pathway to the east.",
            "There is a hallway to the south.",
            "There is a closet off to the side."
        )
    );
    assert_eq!(
        closet.desc(),
        format!(
            "{}\n{}\n{}",
            "Closet",
            "This isn\'t a very large or clean closet.",
            "The door leads back into the room."
        )
    );
    assert_eq!(
        next_room.desc(),
        format!(
            "{}\n{}\n{}",
            "Next Room", "You are in the next room over.", "There is a pathway to the west."
        )
    );
    assert_eq!(
        long_hallway.desc(),
        format!(
            "{}\n{}\n{}",
            "Long Hallway", "You are in a long, dark hallway.", "There is a room to the north"
        )
    );
}
