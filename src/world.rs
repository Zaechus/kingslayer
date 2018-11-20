// Copyright (c) 2018 Maxwell Anderson

use room::Room;

/// Represents a world for the player to explore that consists of an array of Rooms.
/// A World is a graph data structure that encapsulates a collection of Room nodes.
pub struct World {
    pub rooms: Vec<Box<Room>>,
    curr_room: usize,
}

impl World {
    pub fn new(rooms: Vec<Box<Room>>) -> World {
        World {
            rooms,
            curr_room: 0,
        }
    }
    /// index of the current Room
    pub fn curr_room(&self) -> usize {
        self.curr_room
    }
    /// displays description of the current Room
    pub fn look(&self) {
        println!("{}", (*self.rooms[self.curr_room]).desc());
    }
    /// changes the current Room to the target of the current Room's chosen path
    pub fn mv(&mut self, direction: &str) {
        match self.rooms[self.curr_room]
            .paths
            .get(&direction.to_string().clone())
        {
            Some(new_room_name) => {
                let new_room_name = new_room_name.clone();
                let mut new_room: usize = 0;
                for (i, ref x) in self.rooms.iter().enumerate() {
                    if x.name() == new_room_name {
                        new_room = i;
                        break;
                    }
                }
                self.curr_room = new_room;
                self.look();
            }
            None => println!("You cannot go that way."),
        };
    }
}
