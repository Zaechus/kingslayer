use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::item::Item;
use crate::room::Room;

// Represents a world for the player to explore that consists of a grid of Rooms.
// A World is a graph data structure that encapsulates a collection of Room nodes.
#[derive(Serialize, Deserialize)]
pub struct World {
    curr_room: String,
    pub rooms: HashMap<String, Box<Room>>,
}

impl World {
    pub fn curr_room(&self) -> String {
        self.curr_room.clone()
    }

    // displays description of the current Room
    pub fn look(&self) -> String {
        if let Some(room) = self.rooms.get(&self.curr_room) {
            room.desc()
        } else {
            "You are not in a room...".to_string()
        }
    }

    pub fn inspect(&self, name: &str) -> Option<String> {
        if let Some(room) = self.rooms.get(&self.curr_room) {
            if let Some(item) = room.items.get(name) {
                Some(item.inspection())
            } else if let Some(item) = room.paths.get(name) {
                Some(item.inspection())
            } else if let Some(enemy) = room.enemies.get(name) {
                Some(enemy.inspection())
            } else {
                None
            }
        } else {
            None
        }
    }

    // changes the current Room to the target of the current Room's chosen path
    pub fn move_room(&mut self, direction: &str) -> String {
        if let Some(room) = self.rooms.get(&self.curr_room) {
            if let Some(new_room) = room.paths.get(direction) {
                self.curr_room = new_room.name();
                self.look()
            } else {
                "You cannot go that way.".to_string()
            }
        } else {
            "You are not in a room...".to_string()
        }
    }

    // let an Enemy in the current Room take damage
    pub fn harm_enemy(&mut self, enemy: &str, weapon: &str, damage: Option<i32>) -> String {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(nme) = room.enemies.get_mut(enemy) {
                if let Some(dmg) = damage {
                    nme.get_hit(dmg);
                    if nme.hp() > 0 {
                        format!(
                            "You hit the {} with your {} for {} damage.",
                            enemy, weapon, dmg,
                        )
                    } else {
                        let mut res = format!(
                            "You hit the {} with your {} for {} damage. It is dead.\n",
                            enemy, weapon, dmg
                        );
                        if !nme.loot.is_empty() {
                            res.push_str("It dropped:\n");
                            for x in nme.loot.iter() {
                                res.push_str(&format!(" {},", x.1.name()));
                            }
                        }
                        res
                    }
                } else {
                    format!("You do not have the \"{}\". {:?} ", weapon, damage)
                }
            } else {
                format!("There is no \"{}\" here.", enemy)
            }
        } else {
            "You are not in a room...".to_string()
        }
    }

    // move an Item out of the current Room
    pub fn give(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            room.items.remove(name)
        } else {
            None
        }
    }

    pub fn give_from(&mut self, item: &str, container: &str) -> Option<Box<Item>> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(cont) = room.items.get_mut(container) {
                if let Some(ref mut contents) = cont.contents {
                    contents.remove(item)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn give_all(&mut self) -> HashMap<String, Box<Item>> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            let items = room.items.clone();
            room.items.clear();
            room.items.shrink_to_fit();
            items
        } else {
            HashMap::new()
        }
    }

    // insert an Item into the current Room
    pub fn insert(&mut self, cmd: &str, name: &str, item: Option<Box<Item>>) -> String {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(obj) = item {
                room.items.insert(obj.name(), obj);
                match cmd {
                    "throw" => format!("You throw the {} across the room.", name),
                    _ => "Dropped.".to_string(),
                }
            } else {
                format!("You do not have the \"{}\".", name)
            }
        } else {
            "You are not in a room...".to_string()
        }
    }

    // insert an Item into a container Item in the current Room
    pub fn insert_into(&mut self, name: &str, container: &str, item: Option<Box<Item>>) -> String {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(obj) = item {
                if let Some(cont) = room.items.get_mut(container) {
                    if let Some(ref mut contents) = cont.contents {
                        contents.insert(obj.name(), obj);
                        "Placed.".to_string()
                    } else {
                        "You can not put anything in there.".to_string()
                    }
                } else {
                    format!("There is no \"{}\" here.", container)
                }
            } else {
                format!("You do not have the \"{}\".", name)
            }
        } else {
            "You are not in a room...".to_string()
        }
    }
}
