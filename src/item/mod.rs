use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::properties::{IsLocked, IsOpen};

// An object to be interacted with by the user
#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    name: String,
    desc: String,
    inspection: String,
    pub contents: Option<HashMap<String, Box<Item>>>,
    is_locked: Option<IsLocked>,
    is_open: Option<IsOpen>,
}

impl Item {
    pub fn new(name: &str, desc: &str, inspection: &str) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
            inspection: inspection.to_string(),
            contents: None,
            is_locked: None,
            is_open: None,
        }
    }

    pub fn new_container(
        name: &str,
        desc: &str,
        inspection: &str,
        contents: Option<HashMap<String, Box<Item>>>,
        is_locked: Option<IsLocked>,
        is_open: Option<IsOpen>,
    ) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
            inspection: inspection.to_string(),
            contents,
            is_locked,
            is_open,
        }
    }

    pub fn name(&self) -> String {
        match self.contents {
            Some(ref contents) => {
                if !contents.is_empty() {
                    let mut desc = format!("{}; it contains:", self.name);
                    for x in contents.iter() {
                        desc = format!("{}\n    {}", desc, x.1.name());
                    }
                    return desc;
                }
                self.name.clone()
            }
            None => self.name.clone(),
        }
    }

    pub fn desc(&self) -> String {
        match self.contents {
            Some(ref contents) => {
                if !contents.is_empty() {
                    let mut desc = format!("{}\nThe {} contains:", self.desc, self.name);
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

    pub fn inspection(&self) -> String {
        self.inspection.clone()
    }
}
