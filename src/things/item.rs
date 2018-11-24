use things::obj::{Obj, ObjType};

/// A basic type of Obj
pub struct Item {
    name: String,
    desc: String,
}

impl Obj for Item {
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

impl Item {
    pub fn new(name: &str, desc: &str) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
        }
    }
}
