extern crate kinggame1d;

use kinggame1d::cli::Cli;
use kinggame1d::obj::Obj;
use kinggame1d::room::Room;

fn main() {
    let mut rooms: Vec<Box<Room>> = Vec::new();
    rooms.reserve(2);

    // start room
    let iron_sword = Obj::new("There is an iron sword on the ground.");
    let mut start_room_objs = Vec::new();
    start_room_objs.push(Box::new(iron_sword));
    let mut start_room = Box::new(Room::new(
        "Start Room",
        "You stand at the beginning.",
        start_room_objs,
    ));

    // next room
    let wood_block = Obj::new("It's just a wooden block.");
    let mut next_room_objs = Vec::new();
    next_room_objs.push(Box::new(wood_block));
    let mut next_room = Box::new(Room::new(
        "Next Room",
        "You are in the next room over.",
        next_room_objs,
    ));

    start_room.add_path("e", &next_room.name(), "There is a hallway to the east.");
    next_room.add_path("w", &start_room.name(), "There is a hallway to the east.");

    rooms.push(start_room);
    rooms.push(next_room);

    let cli = Cli::new(rooms);
    cli.start();
}
