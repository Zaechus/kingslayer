use std::cell::RefCell;
use std::collections::HashMap;

use rand::Rng;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::item::Item;

#[derive(Serialize, Deserialize)]
pub struct Enemy {
    hp: i32,
    damage: i32,
    name: String,
    desc: String,
    inspection: String,
    is_angry: bool,
    pub loot: RefCell<HashMap<String, Box<Item>>>,
}

impl Enemy {
    pub fn hp(&self) -> i32 {
        self.hp
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn desc(&self) -> String {
        self.desc.clone()
    }

    pub fn inspection(&self) -> String {
        self.inspection.clone()
    }

    pub fn damage(&self) -> i32 {
        rand::thread_rng().gen_range(0, self.damage + 1)
    }

    pub fn is_angry(&self) -> bool {
        self.is_angry
    }

    pub fn get_hit(&mut self, damage: i32) {
        self.is_angry = true;
        self.hp -= damage
    }
}
