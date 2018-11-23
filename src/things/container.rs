use std::collections::HashMap;

use things::obj::Obj;

/// An struct type to represent all objects present in a Room
pub struct Container {
    name: String,
    desc: String,
    /// Items contained if the Item is a container
    pub contents: HashMap<String, Box<Container>>,
}

impl Obj for Container {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn desc(&self) -> String {
        self.desc.clone()
    }
}

impl Container {
    pub fn new(name: &str, desc: &str) -> Container {
        Container {
            name: name.to_owned(),
            desc: desc.to_owned(),
            contents: HashMap::new(),
        }
    }
}
