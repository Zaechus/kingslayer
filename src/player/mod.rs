use std::collections::HashMap;
use std::{thread, time};

use rand::Rng;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::item::Item;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub hp: (i32, i32),
    pub in_combat: bool,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    pub main_hand: Option<Box<Item>>,
    inventory: HashMap<String, Box<Item>>,
}

impl Player {
    pub fn hp(&self) -> i32 {
        self.hp.0
    }

    pub fn hp_cap(&self) -> i32 {
        self.hp.1
    }

    // attack an Enemy with a chosen item in the current Room
    pub fn attack(&mut self) -> Option<i32> {
        if let Some(weapon) = &self.main_hand {
            self.in_combat = true;
            Some(weapon.damage() + ((self.strength - 10) as f32 / 2.0).floor() as i32)
        } else {
            None
        }
    }

    // attack an Enemy with a chosen item in the current Room
    pub fn attack_with(&mut self, weapon: &str) -> Option<i32> {
        if let Some(wpon) = self.inventory.get(weapon) {
            self.in_combat = true;
            Some(wpon.damage() + ((self.strength - 10) as f32 / 2.0).floor() as i32)
        } else {
            None
        }
    }

    // rest for a random amount of time to regain a random amount of HP
    pub fn rest(&mut self) -> String {
        if self.hp() < self.hp_cap() {
            thread::sleep(time::Duration::from_millis(
                rand::thread_rng().gen_range(2000, 5001),
            ));
            let regained_hp = rand::thread_rng().gen_range(1, 7);
            let new_hp = self.hp() + regained_hp;
            if new_hp < self.hp_cap() {
                self.hp = (new_hp, self.hp_cap());
            } else {
                self.hp = (self.hp_cap(), self.hp_cap());
            }
            format!(
                "You regained {} HP for a total of ({} / {}) HP.",
                regained_hp,
                self.hp(),
                self.hp_cap()
            )
        } else {
            "You already have full health.".to_string()
        }
    }

    pub fn inventory(&self) -> String {
        if self.inventory.is_empty() {
            "You are empty-handed.".to_string()
        } else {
            let mut items_carried = String::new();
            if let Some(weapon) = &self.main_hand {
                items_carried.push_str(&format!("Main hand: {}\n", weapon.name()));
            }
            items_carried.push_str("You are carrying:");
            for x in self.inventory.iter() {
                items_carried = format!("{}\n  {}", items_carried, x.1.name());
            }
            items_carried
        }
    }

    pub fn status(&self) -> String {
        format!("You have ({} / {}) HP.", self.hp(), self.hp_cap())
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp = (self.hp.0 - damage, self.hp.1);
    }

    pub fn inspect(&self, name: &str) -> Option<String> {
        if name == "me" || name == "self" || name == "myself" {
            Some(self.status())
        } else if let Some(item) = self.inventory.get(name) {
            Some(item.inspection())
        } else {
            None
        }
    }

    // take an Item from the current Room
    pub fn take(&mut self, name: &str, item: Option<Box<Item>>) -> String {
        if let Some(obj) = item {
            self.inventory.insert(name.to_string(), obj);
            "Taken.".to_string()
        } else {
            format!("There is no \"{}\" here.", name)
        }
    }

    // take an Item from a container Item in the inventory
    pub fn take_from(&mut self, item: &str, container: &str) -> String {
        if let Some(cont) = self.inventory.get_mut(container) {
            if let Some(ref mut contents) = cont.contents {
                if let Some(itm) = contents.remove(item) {
                    self.inventory.insert(item.to_string(), itm);
                    "Taken.".to_string()
                } else {
                    format!("There is no \"{}\" inside of the \"{}\".", item, container)
                }
            } else {
                format!("You cannot put anything \"{}\".", container)
            }
        } else {
            format!("You do not have the \"{}\".", container)
        }
    }

    // take all Items in the current Room
    pub fn take_all(&mut self, items: HashMap<String, Box<Item>>) -> String {
        self.inventory.extend(items);
        "Taken.".to_string()
    }

    // remove an item from inventory and into the current Room
    pub fn remove(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.inventory.remove(name) {
            Some(item)
        } else if let Some(item) = self.main_hand.take() {
            Some(item)
        } else {
            None
        }
    }

    // equip an item to fight with
    pub fn equip(&mut self, weapon: &str) -> String {
        if let Some(item) = self.inventory.remove(weapon) {
            if let Some(wpon) = self.main_hand.take() {
                self.take(&wpon.name(), Some(wpon));
            }
            self.main_hand = Some(item);
            "Equipped.".to_string()
        } else {
            format!("You do not have the \"{}\".", weapon)
        }
    }

    pub fn put_in(&mut self, item: &str, container: &str) -> String {
        if let Some(itm) = self.inventory.remove(item) {
            if let Some(cont) = self.inventory.get_mut(container) {
                if let Some(ref mut contents) = cont.contents {
                    contents.insert(item.to_string(), itm);
                    "Placed.".to_string()
                } else {
                    format!("You cannot put anything in the {}.", container)
                }
            } else {
                format!("You do not have the \"{}\".", container)
            }
        } else {
            format!("You do not have the \"{}\".", item)
        }
    }
}
