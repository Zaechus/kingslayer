use std::collections::HashMap;

use things::obj::{Obj, ObjType};

/// A type of Obj that can container other Objs
pub struct Container {
    name: String,
    desc: String,
    /// Objs contained within the Container
    pub contents: HashMap<String, Box<Obj>>,
}

impl Obj for Container {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn desc(&self) -> String {
        self.desc.clone()
    }
    fn objtype(&self) -> ObjType {
        ObjType::Container
    }
}

impl Container {
    pub fn new(name: &str, desc: &str) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            contents: HashMap::new(),
        }
    }
}
