/// An struct type to represent all objects present in a Room
pub struct Obj {
    desc: String,
}

impl Obj {
    pub fn new(desc: &str) -> Obj {
        Obj {
            desc: desc.to_owned(),
        }
    }
    /// return
    pub fn desc(&self) -> String {
        self.desc.clone()
    }
}
