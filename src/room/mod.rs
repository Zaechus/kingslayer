use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::enemy::Enemy;
use crate::item::Item;
use crate::pathway::Pathway;

// A section of the world connected by paths
#[derive(Serialize, Deserialize)]
pub struct Room {
    name: String,
    desc: String,
    // pathways to other Rooms
    pub paths: HashMap<String, Pathway>,
    // Enemies contained within the Room
    pub enemies: HashMap<String, Box<Enemy>>,
    // Items contained within the Room
    pub items: HashMap<String, Box<Item>>,
}

impl Room {
    // compiles all descriptions in the Room for printing
    pub fn desc(&self) -> String {
        let mut desc = format!("{}\n{}", self.name, self.desc);
        for x in self.paths.iter() {
            desc.push_str(&format!("\n{}", &(x.1).desc()));
        }
        for x in self.enemies.iter() {
            desc.push_str(&format!("\n{}", &x.1.desc()));
        }
        for x in self.items.iter() {
            desc.push_str(&format!("\n{}", &x.1.desc()));
        }
        desc
    }
}
