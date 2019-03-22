use std::collections::HashMap;

use rand::Rng;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::entities::Item;

#[derive(Serialize, Deserialize)]
pub struct Enemy {
    hp: i32,
    damage: i32,
    name: String,
    desc: String,
    inspection: String,
    is_angry: bool,
    loot: HashMap<String, Box<Item>>,
}

impl Enemy {
    pub fn hp(&self) -> i32 {
        self.hp
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn desc(&self) -> &String {
        &self.desc
    }

    pub fn inspection(&self) -> &String {
        &self.inspection
    }

    pub fn damage(&self) -> i32 {
        rand::thread_rng().gen_range(0, self.damage + 1)
    }

    pub fn is_angry(&self) -> bool {
        self.is_angry
    }

    pub fn loot(&self) -> &HashMap<String, Box<Item>> {
        &self.loot
    }

    pub fn get_hit(&mut self, damage: i32) {
        self.is_angry = true;
        self.hp -= damage;
    }

    pub fn drop_loot(&self) -> HashMap<String, Box<Item>> {
        self.loot.clone()
    }
}
