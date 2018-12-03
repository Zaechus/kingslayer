use std::collections::HashMap;

// An object to be interacted with by the user
#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    desc: String,
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
                if !contents.is_empty() {
                    let mut desc = format!("{} contains:", self.name);
                    for x in contents.iter() {
                        desc = format!("{}\n  {}", desc, x.1.name());
                    }
                    return desc;
                }
                self.desc.clone()
            }
            None => self.desc.clone(),
        }
    }
}
