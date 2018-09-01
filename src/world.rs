use room::Room;

/// Represents a world for the player to explore that consists of an array of rooms.
/// A World is a graph data structure that encapsulates a collection of Room nodes.
pub struct World {
    pub rooms: Vec<Room>,
}
