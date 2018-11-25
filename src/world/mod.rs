use room::Room;

/// Represents a world for the player to explore that consists of an array of Rooms.
/// A World is a graph data structure that encapsulates a collection of Room nodes.
pub struct World {
    curr_room: usize,
    pub rooms: Vec<Box<Room>>,
}

impl World {
    pub fn new(rooms: Vec<Box<Room>>) -> Self {
        Self {
            curr_room: 0,
            rooms,
        }
    }
    /// index of the current Room
    pub fn curr_room(&self) -> usize {
        self.curr_room
    }
    /// displays description of the current Room
    pub fn look(&self) -> String {
        (*self.rooms[self.curr_room]).desc()
    }
    /// changes the current Room to the target of the current Room's chosen path
    pub fn mv(&mut self, direction: &str) {
        match self.rooms[self.curr_room]
            .paths
            .get(&direction.to_string().clone())
        {
            Some(new_room_name) => {
                self.curr_room = self
                    .rooms
                    .iter()
                    .position(|ref r| &r.name() == new_room_name)
                    .unwrap();
                println!("{}", self.look());
            }
            None => println!("You cannot go that way."),
        };
    }
}
