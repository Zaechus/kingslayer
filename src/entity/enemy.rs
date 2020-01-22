use rand::Rng;

use serde::{Deserialize, Serialize};

use super::{Entity, Item};
use crate::{dice_roll, types::Items};

#[derive(Debug, Serialize, Deserialize)]
pub struct Enemy {
    name: String,
    desc: String,
    inspect: String,
    hp: i32,
    xp: u32,
    damage: u32,
    is_angry: bool,
    #[serde(default)]
    loot: Items,
}

impl Enemy {
    pub fn new_rats(is_angry: bool) -> Self {
        Self {
            name: String::from("swarm rats"),
            desc: String::from("A swarm of rats raves along the floor."),
            inspect: String::from("The creatures chatter and scrape viciously."),
            hp: 24,
            xp: 50,
            damage: 3,
            is_angry,
            loot: Items::new(),
        }
    }

    pub fn new_pirate(is_angry: bool) -> Self {
        Self {
            name: String::from("pirate"),
            desc: String::from("There is a pirate here."),
            inspect: String::from("The pirate is armed and smells vile."),
            hp: dice_roll(3, 8) as i32,
            xp: 50,
            damage: 10,
            is_angry,
            loot: Items::new(),
        }
    }

    pub fn with_item(mut self, item: Item) -> Self {
        self.loot.push(Box::new(item));
        self
    }

    pub const fn xp(&self) -> u32 {
        self.xp
    }

    pub fn damage(&self) -> u32 {
        rand::thread_rng().gen_range(1, self.damage + 1)
    }

    pub const fn is_angry(&self) -> bool {
        self.is_angry
    }

    pub fn make_angry(&mut self) {
        self.is_angry = true;
    }

    pub const fn loot(&self) -> &Items {
        &self.loot
    }

    pub fn get_hit(&mut self, damage: i32) {
        self.make_angry();
        self.hp -= damage;
    }

    pub const fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn drop_loot(&mut self) -> Items {
        self.loot.drain(0..).collect()
    }
}

impl Entity for Enemy {
    fn name(&self) -> &str {
        &self.name
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn inspect(&self) -> &str {
        &self.inspect
    }
}
