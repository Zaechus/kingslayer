use items::obj::Obj;

/// An struct type to represent all objects present in a Room
#[derive(Debug)]
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
}

impl Item {
    pub fn new(name: &str, desc: &str) -> Item {
        Item {
            name: name.to_owned(),
            desc: desc.to_owned(),
        }
    }
}
