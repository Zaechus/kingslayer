use serde::{Deserialize, Serialize};

use super::{Entity, Item};
use crate::{dice_roll, types::Items};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub fn new(name: &str, inspect: &str, is_angry: bool) -> Self {
        Self {
            name: name.to_string(),
            desc: format!("There is a {} here.", name),
            inspect: inspect.to_string(),
            hp: 1,
            xp: 0,
            damage: 1,
            is_angry,
            loot: Items::new(),
        }
    }
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
            hp: dice_roll(3, 8) as i32 + 8,
            xp: 50,
            damage: 10,
            is_angry,
            loot: Items::new(),
        }
    }

    pub fn with_desc(mut self, desc: &str) -> Self {
        self.desc = String::from(desc);
        self
    }
    pub fn with_hp(mut self, hp: i32) -> Self {
        self.hp = hp;
        self
    }
    pub fn with_xp(mut self, xp: u32) -> Self {
        self.xp = xp;
        self
    }
    pub fn with_damage(mut self, damage: u32) -> Self {
        self.damage = damage;
        self
    }
    pub fn with_item(mut self, item: Item) -> Self {
        self.loot.push(Box::new(item));
        self
    }

    pub const fn xp(&self) -> u32 {
        self.xp
    }

    pub fn damage(&self) -> u32 {
        dice_roll(1, self.damage)
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

    pub fn get_hit(&mut self, damage: u32) {
        self.make_angry();
        self.hp -= damage as i32;
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
