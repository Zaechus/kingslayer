use rand::Rng;

use serde_derive::{Deserialize, Serialize};

use crate::types::ItemMap;

// An object to be interacted with by the user
#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    name: String,
    desc: String,
    inspection: String,
    is_closed: Option<bool>,
    is_locked: Option<bool>,
    damage: Option<i32>,
    armor_class: Option<i32>,
    contents: Option<ItemMap>,
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

    pub fn inspection(&self) -> &String {
        &self.inspection
    }

    pub fn is_weapon(&self) -> bool {
        self.damage.is_some()
    }

    pub fn damage(&self) -> i32 {
        if let Some(damage) = self.damage {
            rand::thread_rng().gen_range(1, damage + 1)
        } else {
            0
        }
    }

    pub fn armor_class(&self) -> i32 {
        if let Some(ac) = self.armor_class {
            ac
        } else {
            10
        }
    }

    pub fn contents_mut(&mut self) -> &mut Option<ItemMap> {
        &mut self.contents
    }
}
