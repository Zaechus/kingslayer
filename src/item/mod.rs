use std::collections::HashMap;

use rand::Rng;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::properties::{IsLocked, IsOpen};

// An object to be interacted with by the user
#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    name: String,
    desc: String,
    inspection: String,
    is_locked: Option<IsLocked>,
    is_open: Option<IsOpen>,
    damage: Option<i32>,
    pub contents: Option<HashMap<String, Box<Item>>>,
}

impl Item {
    pub fn name(&self) -> String {
        if let Some(ref contents) = self.contents {
            if !contents.is_empty() {
                let mut desc = format!("{}; it contains:", self.name);
                for x in contents.iter() {
                    desc = format!("{}\n    {}", desc, x.1.name());
                }
                return desc;
            }
            self.name.clone()
        } else {
            self.name.clone()
        }
    }

    pub fn desc(&self) -> String {
        if let Some(ref contents) = self.contents {
            if !contents.is_empty() {
                let mut desc = format!("{}\nThe {} contains:", self.desc, self.name);
                for x in contents.iter() {
                    desc = format!("{}\n  {}", desc, x.1.name());
                }
                desc
            } else {
                self.desc.clone()
            }
        } else {
            self.desc.clone()
        }
    }

    pub fn inspection(&self) -> String {
        self.inspection.clone()
    }

    pub fn damage(&self) -> i32 {
        if let Some(damage) = self.damage {
            rand::thread_rng().gen_range(1, damage + 1)
        } else {
            rand::thread_rng().gen_range(0, 2)
        }
    }
}
