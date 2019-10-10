use rand::Rng;

use serde_derive::{Deserialize, Serialize};

use crate::entity::{Closeable, Entity};
use crate::types::ItemMap;

// An object to be interacted with by the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    name: String,
    desc: String,
    inspection: String,
    is_closed: Option<bool>,
    is_locked: Option<bool>,
    damage: Option<u32>,
    armor_class: Option<u32>,
    contents: Option<ItemMap>,
}

impl Item {
    pub fn long_name(&self) -> String {
        if let Some(ref contents) = self.contents {
            if !contents.is_empty() && !self.is_closed.unwrap_or(false) {
                let mut desc = format!("{}; it contains:", self.name);
                for item in contents.values() {
                    desc = format!("{}\n    {}", desc, item.name());
                }
                desc
            } else {
                self.name.clone()
            }
        } else {
            self.name.clone()
        }
    }

    pub fn long_desc(&self) -> String {
        if let Some(ref contents) = self.contents {
            if !contents.is_empty() && !self.is_closed.unwrap_or(false) {
                let mut desc = format!("{}\nThe {} contains:", self.desc, self.name);
                for item in contents.values() {
                    desc = format!("{}\n  {}", desc, item.name());
                }
                desc
            } else {
                self.desc.clone()
            }
        } else {
            self.desc.clone()
        }
    }

    pub fn is_weapon(&self) -> bool {
        self.damage.is_some()
    }

    pub fn damage(&self) -> u32 {
        if let Some(damage) = self.damage {
            rand::thread_rng().gen_range(1, damage + 1)
        } else {
            0
        }
    }

    pub fn armor_class(&self) -> Option<u32> {
        self.armor_class
    }

    pub fn contents_mut(&mut self) -> &mut Option<ItemMap> {
        &mut self.contents
    }

    pub fn is_closed(&self) -> Option<bool> {
        self.is_closed
    }
}

impl Entity for Item {
    fn name(&self) -> &String {
        &self.name
    }

    fn desc(&self) -> &String {
        &self.desc
    }

    fn inspection(&self) -> &String {
        &self.inspection
    }
}

impl Closeable for Item {
    fn open(&mut self) {
        if self.is_closed.is_some() {
            self.is_closed = Some(false)
        }
    }
    fn close(&mut self) {
        if self.is_closed.is_some() {
            self.is_closed = Some(true)
        }
    }
}
