use room::Room;

/// Represents a world for the player to explore that consists of an array of rooms.
/// A World is a graph data structure that encapsulates a collection of Room nodes.
pub struct World {
    pub rooms: Vec<Box<Room>>,
    curr_room: usize,
}

impl World {
    pub fn new(rooms: Vec<Box<Room>>) -> World {
        World {
            rooms: rooms,
            curr_room: 0,
        }
    }
    pub fn look(&self) {
        println!("{}", (*self.rooms[self.curr_room]).desc());
    }
    pub fn mv(&mut self, direction: &str) {
        let tmp = self.rooms[self.curr_room].paths.get(direction.clone());
        let tmp = match tmp {
            Some(s) => s,
            None => "",
        };
        let mut new_room: usize = 0;
        for (i, ref x) in self.rooms.iter().enumerate() {
            if x.name() == tmp {
                new_room = i;
            }
        }
        self.curr_room = new_room;
        self.look();
    }
}
