use rand::Rng;

use serde_derive::{Deserialize, Serialize};

use crate::types::ItemMap;

#[derive(Serialize, Deserialize)]
pub struct Enemy {
    hp: i32,
    xp: i32,
    damage: i32,
    name: String,
    desc: String,
    inspection: String,
    is_angry: bool,
    loot: ItemMap,
}

impl Enemy {
    pub fn xp(&self) -> i32 {
        self.xp
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

    pub fn make_angry(&mut self) {
        self.is_angry = true;
    }

    pub fn loot(&self) -> &ItemMap {
        &self.loot
    }

    pub fn get_hit(&mut self, damage: i32) {
        self.make_angry();
        self.hp -= damage;
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn drop_loot(&self) -> ItemMap {
        self.loot.clone()
    }
}