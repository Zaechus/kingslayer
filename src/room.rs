use std::collections::HashMap;

use obj::Obj;

/// A node found within a World that is connected by Paths
pub struct Room {
    name: String,
    desc: String,
    pub paths: HashMap<String, String>,
    objs: Vec<Box<Obj>>,
}

impl Room {
    pub fn new(name: &str, desc: &str, objs: Vec<Box<Obj>>) -> Room {
        Room {
            name: name.to_owned(),
            desc: desc.to_owned(),
            paths: HashMap::new(),
            objs: objs,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn desc(&self) -> String {
        let mut desc = format!("{}\n{}\n", self.name, self.desc);
        for x in self.objs.iter() {
            desc.push_str(&x.desc());
        }
        desc
    }
    pub fn add_path(&mut self, dir: &str, room: &String, desc: &str) {
        self.paths.insert(dir.to_owned(), room.clone());
        self.desc.push_str(format!("\n{}", desc).as_str());
    }
}
