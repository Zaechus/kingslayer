use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

use item::Item;

/// A section of the world connected by paths
#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    name: String,
    desc: String,
    /// pathways to other Rooms
    pub paths: HashMap<String, (String, String)>,
    /// Items contained within the Room
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
    /// name of the Room
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// compiles all descriptions in the Room for printing
    pub fn desc(&self) -> String {
        let mut desc = format!("{}\n{}", self.name, self.desc);
        for x in self.items.iter() {
            desc.push_str(&format!("\n{}", &x.1.desc()));
        }
        for x in self.paths.iter() {
            desc.push_str(&format!("\n{}", &(x.1).1));
        }
        desc
    }
    /// add path to another Room
    pub fn add_path(&mut self, direction: &str, name: &str, desc: &str) {
        self.paths
            .insert(direction.to_owned(), (name.to_owned(), desc.to_owned()));
    }
}

#[cfg(test)]
mod tests;
