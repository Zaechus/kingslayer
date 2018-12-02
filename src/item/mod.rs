use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

/// An object to be interacted with by the user
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    name: String,
    desc: String,
    /// Items contained within a container Item
    pub contents: Option<HashMap<String, Box<Item>>>,
}

impl Item {
    pub fn new(name: &str, desc: &str) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            contents: None,
        }
    }
    pub fn new_container(
        name: &str,
        desc: &str,
        contents: Option<HashMap<String, Box<Item>>>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            contents,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn desc(&self) -> String {
        match self.contents {
            Some(ref contents) => {
                let mut desc = format!("{} contains:\n", self.name);
                for x in contents.iter() {
                    desc = format!("{}  {}\n", desc, x.1.name());
                }
                desc
            }
            None => self.desc.clone(),
        }
    }
}
