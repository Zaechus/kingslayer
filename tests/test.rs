// Copyright (c) 2018 Maxwell Anderson

extern crate kinggame1d;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use kinggame1d::{
        cli::Cli, room::Room, things::container::Container, things::item::Item, things::obj::Obj,
    };

    #[test]
    fn setup_cli_works() {
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
        start_room.add_path(
            "s",
            &long_hallway.name(),
            "There is a hallway to the south.",
        );
        long_hallway.add_path("n", &start_room.name(), "There is a room to the north");

        let rooms: Vec<Box<Room>> = vec![start_room, long_hallway];

        let _cli = Cli::new(rooms);
    }
}
