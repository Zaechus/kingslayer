// Copyright (c) 2018 Maxwell Anderson

use std::collections::HashMap;

use things::obj::Obj;

/// A node found within a World that is connected by paths
pub struct Room {
    name: String,
    desc: String,
    /// pathways to other Rooms
    pub paths: HashMap<String, String>,
    /// Items contained within the Room
    pub items: HashMap<String, Box<Obj>>,
}

impl Room {
    pub fn new(name: &str, desc: &str, items: HashMap<String, Box<Obj>>) -> Room {
        Room {
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
        for x in &self.items {
            desc.push_str(&format!("\n{}", &x.1.desc()));
        }
        desc
    }
    /// add path directive to another Room
    pub fn add_path(&mut self, dir: &str, room: &str, desc: &str) {
        self.paths.insert(dir.to_owned(), room.to_string().clone());
        self.desc.push_str(format!("\n{}", desc).as_str());
    }
}

#[cfg(test)]
mod tests;
