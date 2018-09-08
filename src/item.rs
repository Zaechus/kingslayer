/// An struct type to represent all objects present in a Room
pub struct Item {
    name: String,
    desc: String,
    is_container: bool,
    pub contents: Vec<Item>,
}

impl Item {
    pub fn new(name: &str, desc: &str, is_container: bool) -> Item {
        Item {
            name: name.to_owned(),
            desc: desc.to_owned(),
            is_container: is_container,
            contents: Vec::new(),
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn desc(&self) -> String {
        self.desc.clone()
    }
    pub fn is_container(&self) -> bool {
        self.is_container
    }
}
