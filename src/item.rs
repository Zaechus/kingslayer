use std::collections::HashMap;

/// An struct type to represent all objects present in a Room
pub struct Item {
    name: String,
    desc: String,
    is_container: bool,
    /// Items contained if the Item is a container
    pub contents: HashMap<String, Box<Item>>,
}

impl Item {
    pub fn new(name: &str, desc: &str, is_container: bool) -> Item {
        Item {
            name: name.to_owned(),
            desc: desc.to_owned(),
            is_container,
            contents: HashMap::new(),
        }
    }
    /// the name of the Item
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// the game description of the Item
    pub fn desc(&self) -> String {
        self.desc.clone()
    }
    /// if the Item is classified as a 'container'
    /// and is allowed to hold other Items in contents
    pub fn is_container(&self) -> bool {
        self.is_container
    }
}
