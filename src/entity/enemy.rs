use rand::Rng;

use serde_derive::{Deserialize, Serialize};

use crate::entity::Entity;
use crate::types::ItemMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Enemy {
    hp: i32,
    xp: u32,
    damage: u32,
    name: String,
    desc: String,
    inspection: String,
    is_angry: bool,
    loot: ItemMap,
}

impl Enemy {
    pub fn xp(&self) -> u32 {
        self.xp
    }

    pub fn damage(&self) -> u32 {
        rand::thread_rng().gen_range(1, self.damage + 1)
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

    pub fn get_hit(&mut self, damage: u32) {
        self.make_angry();
        self.hp -= damage as i32;
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn drop_loot(&mut self) -> ItemMap {
        self.loot.drain().collect()
    }
}

impl Entity for Enemy {
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
