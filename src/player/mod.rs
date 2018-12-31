use std::collections::HashMap;
use std::{thread, time};

use rand::Rng;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::item::Item;
use crate::world::World;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub hp: (i32, i32),
    pub in_combat: bool,
    main_hand: Option<String>,
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
    pub fn attack(&mut self, world: &mut World, enemy: &str, weapon: &str) -> String {
        let curr_room = &world.curr_room();

        match world.rooms.get_mut(curr_room) {
            Some(room) => match room.enemies.get_mut(enemy) {
                Some(nme) => match self.inventory.get(weapon) {
                    Some(wpon) => {
                        let dmg = wpon.damage();
                        nme.get_hit(dmg);
                        self.in_combat = true;
                        if nme.hp() > 0 {
                            format!(
                                "You hit the {} with your {} for {} damage.",
                                enemy, weapon, dmg,
                            )
                        } else {
                            self.in_combat = false;
                            format!(
                                "You hit the {} with your {} for {} damage. It is dead.",
                                enemy, weapon, dmg
                            )
                        }
                    }
                    None => format!("You do not have the \"{}\".", weapon),
                },
                None => format!("There is no \"{}\" here.", enemy),
            },
            None => "You are not in a room...".to_string(),
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

    // returns inventory contents
    pub fn inventory(&self) -> String {
        if self.inventory.is_empty() {
            "You are empty-handed.".to_string()
        } else {
            let mut items_carried = String::from("You are carrying:");
            for x in self.inventory.iter() {
                items_carried = format!("{}\n  {}", items_carried, x.1.name());
            }
            items_carried
        }
    }

    // returns HP
    pub fn status(&self) -> String {
        format!("You have ({} / {}) HP.", self.hp(), self.hp_cap())
    }

    // returns the special properties of an object or path
    pub fn inspect(&self, world: &World, name: &str) -> String {
        if name == "me" || name == "self" || name == "myself" {
            self.status()
        } else {
            let curr_room = &world.curr_room();

            match world.rooms.get(curr_room) {
                Some(room) => match room.items.get(name) {
                    Some(item) => item.inspection(),

                    None => match self.inventory.get(name) {
                        Some(item) => item.inspection(),

                        None => match room.paths.get(name) {
                            Some(item) => item.inspection(),

                            None => match room.enemies.get(name) {
                                Some(enemy) => enemy.inspection(),
                                None => format!("There is no \"{}\" here.", name),
                            },
                        },
                    },
                },
                None => "You are not in a room...".to_string(),
            }
        }
    }

    // take an Item from the current Room into the inventory
    pub fn take(&mut self, world: &mut World, name: &str) -> String {
        let curr_room = &world.curr_room();

        let taken = match world.rooms.get_mut(curr_room) {
            Some(room) => room.items.remove(name),
            None => None,
        };
        match taken {
            Some(ob) => {
                self.inventory.insert(ob.name(), ob);
                "Taken.".to_string()
            }
            None => format!("There is no \"{}\" here.", name),
        }
    }

    // take all Items in the current Room
    pub fn take_all(&mut self, world: &mut World) -> String {
        let curr_room = &world.curr_room();
        let mut taken_str = String::new();

        if let Some(room) = world.rooms.get_mut(curr_room) {
            for item in &room.items {
                self.inventory.insert(item.0.clone(), item.1.clone());
                taken_str.push_str("Taken. ");
            }
            room.items.clear();
            room.items.shrink_to_fit();
        }
        if taken_str.is_empty() {
            "There is nothing here to take.".to_string()
        } else {
            taken_str
        }
    }

    // take an item from a container in the room or inventory
    pub fn take_from(&mut self, world: &mut World, item: &str, container: &str) -> String {
        let curr_room = &world.curr_room();

        let taken = match world.rooms.get_mut(curr_room) {
            Some(room) => match room.items.get_mut(container) {
                Some(cont) => match cont.contents {
                    Some(ref mut contents) => contents.remove(item),
                    None => None,
                },
                None => match self.inventory.get_mut(container) {
                    Some(cont2) => match cont2.contents {
                        Some(ref mut contents) => contents.remove(item),
                        None => None,
                    },
                    None => return format!("There is no \"{}\" here.", container),
                },
            },
            None => return format!("There is no \"{}\" here.", container),
        };
        match taken {
            Some(ob) => {
                self.inventory.insert(ob.name(), ob);
                "Taken.".to_string()
            }
            None => format!("There is no \"{}\" here.", item),
        }
    }

    // remove an Item from inventory into the current Room
    pub fn remove(&mut self, world: &mut World, cmd: &str, name: &str) -> String {
        let curr_room = &world.curr_room();

        let dropped = self.inventory.remove(name);
        match dropped {
            Some(obj) => {
                if let Some(room) = world.rooms.get_mut(curr_room) {
                    room.items.insert(obj.name(), obj);
                    match cmd {
                        "throw" => format!("You throw the {} across the room.", name),
                        _ => "Dropped.".to_string(),
                    }
                } else {
                    String::new()
                }
            }
            None => format!("You don't have the \"{}\".", name),
        }
    }

    // place an Item into a container Item
    pub fn put_in(&mut self, world: &mut World, item: &str, container: &str) -> String {
        let curr_room = &world.curr_room();

        let placed = self.inventory.remove(item);
        match placed {
            Some(obj) => {
                if let Some(room) = world.rooms.get_mut(curr_room) {
                    match room.items.get_mut(container) {
                        Some(cont) => match cont.contents {
                            Some(ref mut contents) => {
                                contents.insert(obj.name(), obj);
                                "Placed.".to_string()
                            }
                            None => {
                                self.inventory.insert(obj.name(), obj);
                                format!("You can't place anything in the {}.", container)
                            }
                        },
                        None => match self.inventory.get_mut(container) {
                            Some(cont) => match cont.contents {
                                Some(ref mut contents) => {
                                    contents.insert(obj.name(), obj);
                                    "Placed.".to_string()
                                }
                                None => {
                                    self.inventory.insert(obj.name(), obj);
                                    format!("You can't place anything in the {}.", container)
                                }
                            },
                            None => {
                                self.inventory.insert(obj.name(), obj);
                                format!("There is no \"{}\" here.", container)
                            }
                        },
                    }
                } else {
                    String::new()
                }
            }
            None => format!("You don't have the \"{}\".", item),
        }
    }

    // equip an item to fight with
    pub fn equip(&self, weapon: &str) -> String {
        format!("TODO: equip \"{}\"", weapon)
    }
}
