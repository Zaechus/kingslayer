use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::room::Room;

// Represents a world for the player to explore that consists of a grid of Rooms.
// A World is a graph data structure that encapsulates a collection of Room nodes.
#[derive(Serialize, Deserialize)]
pub struct World {
    curr_room: String,
    pub rooms: HashMap<String, Box<Room>>,
}

impl World {
    pub fn new(curr_room: &str, rooms: HashMap<String, Box<Room>>) -> Self {
        Self {
            curr_room: curr_room.to_string(),
            rooms,
        }
    }

    pub fn curr_room(&self) -> String {
        self.curr_room.clone()
    }

    // displays description of the current Room
    pub fn look(&self) -> String {
        match self.rooms.get(&self.curr_room) {
            Some(room) => room.desc(),
            None => "You are not in a room...".to_string(),
        }
    }

    // changes the current Room to the target of the current Room's chosen path
    pub fn move_room(&mut self, direction: &str) -> String {
        match self.rooms.get(&self.curr_room) {
            Some(room) => match room.paths.get(direction) {
                Some(new_room_name) => {
                    self.curr_room = new_room_name.name();
                    self.look()
                }
                None => "You cannot go that way.".to_string(),
            },
            None => "You are not in a room...".to_string(),
        }
    }
}
