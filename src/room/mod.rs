use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::item::Item;
use crate::pathway::Pathway;
use crate::properties::IsLocked;
use crate::properties::IsOpen;

// A section of the world connected by paths
#[derive(Serialize, Deserialize)]
pub struct Room {
    name: String,
    desc: String,
    // pathways to other Rooms
    pub paths: HashMap<String, Pathway>,
    // Items contained within the Room
    pub items: HashMap<String, Box<Item>>,
}

impl Room {
    pub fn new(name: &str, desc: &str, items: HashMap<String, Box<Item>>) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            paths: HashMap::new(),
            items,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    // compiles all descriptions in the Room for printing
    pub fn desc(&self) -> String {
        let mut desc = format!("{}\n{}", self.name, self.desc);
        for x in self.paths.iter() {
            desc.push_str(&format!("\n{}", &(x.1).desc()));
        }
        for x in self.items.iter() {
            desc.push_str(&format!("\n{}", &x.1.desc()));
        }
        desc
    }

    // add Pathway pointing to another Room
    pub fn add_path(
        &mut self,
        direction: &str,
        name: &str,
        desc: &str,
        inspection: &str,
        is_open: IsOpen,
        is_locked: IsLocked,
    ) {
        self.paths.insert(
            direction.to_owned(),
            Pathway::new(name, desc, inspection, is_open, is_locked),
        );
    }
}

#[cfg(test)]
mod tests;
