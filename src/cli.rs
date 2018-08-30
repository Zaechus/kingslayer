pub struct Cli {
    name: String,
    rooms: Vec<String>,
}

impl Cli {
    pub fn new(name: String, rooms: Vec<String>) -> Cli {
        Cli {
            name: name,
            rooms: rooms,
        }
    }
    pub fn start(&self) {
        loop {}
    }
}
