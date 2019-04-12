use serde_derive::{Deserialize, Serialize};

use crate::{
    entities::{Item, Room},
    types::{CmdResult, ItemMap, RoomMap},
};

// Represents a world for the player to explore that consists of a grid of Rooms.
// A World is a graph data structure that encapsulates a collection of Room nodes.
#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    curr_room: String,
    rooms: RoomMap,
}

impl World {
    pub fn get_curr_room(&self) -> &Room {
        match self.rooms.get(&self.curr_room) {
            Some(room) => room,
            None => {
                panic!("ERROR: You are not in a valid room (The world creator should fix this).")
            }
        }
    }

    pub fn get_curr_room_mut(&mut self) -> &mut Room {
        match self.rooms.get_mut(&self.curr_room) {
            Some(room) => room,
            None => {
                panic!("ERROR: You are not in a valid room (The world creator should fix this).")
            }
        }
    }

    // displays description of the current Room
    pub fn look(&self) -> CmdResult {
        CmdResult::new(true, self.get_curr_room().desc())
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if let Some(item) = self.get_curr_room().items().get(name) {
            Some(CmdResult::new(true, item.inspection().to_string()))
        } else if let Some(item) = self.get_curr_room().paths().get(name) {
            Some(CmdResult::new(true, item.inspection().to_string()))
        } else if let Some(enemy) = self.get_curr_room().enemies().get(name) {
            Some(CmdResult::new(true, enemy.inspection().to_string()))
        } else {
            None
        }
    }

    // changes the current Room to the target of the current Room's chosen path
    pub fn move_room(&mut self, direction: &str) -> CmdResult {
        if let Some(new_room) = self.get_curr_room().paths().get(direction) {
            if new_room.is_locked() == Some(true) {
                CmdResult::new(true, "The way is locked.".to_string())
            } else if new_room.is_closed() == Some(true) {
                CmdResult::new(true, "The way is closed.".to_string())
            } else {
                for e in self.get_curr_room().enemies().values() {
                    if e.is_angry() {
                        return CmdResult::new(false, "Enemies bar your way.".to_string());
                    }
                }
                self.curr_room = new_room.target().to_string();
                self.look()
            }
        } else {
            CmdResult::new(false, "You cannot go that way.".to_string())
        }
    }

    pub fn open_path(&mut self, path: &str) -> CmdResult {
        if let Some(p) = self.get_curr_room_mut().paths_mut().get_mut(path) {
            if p.is_closed() == Some(true) {
                p.open();
                CmdResult::new(true, "Opened.".to_string())
            } else {
                CmdResult::new(false, format!("The {} is already opened.", path))
            }
        } else {
            CmdResult::new(false, format!("There is no \"{}\".", path))
        }
    }

    pub fn close_path(&mut self, path: &str) -> CmdResult {
        if let Some(p) = self.get_curr_room_mut().paths_mut().get_mut(path) {
            if p.is_closed() == Some(true) {
                CmdResult::new(false, format!("The {} is already closed.", path))
            } else {
                p.close();
                CmdResult::new(true, "Closed.".to_string())
            }
        } else {
            CmdResult::new(false, format!("There is no \"{}\".", path))
        }
    }

    // let an Enemy in the current Room take damage
    pub fn harm_enemy(&mut self, damage: Option<u32>, enemy_name: &str, weapon: &str) -> CmdResult {
        if let Some(enemy) = self.get_curr_room_mut().enemies_mut().get_mut(enemy_name) {
            if let Some(damage) = damage {
                enemy.get_hit(damage);
                if enemy.is_alive() {
                    CmdResult::new(
                        true,
                        format!(
                            "You hit the {} with your {} for {} damage.",
                            enemy_name, weapon, damage,
                        ),
                    )
                } else {
                    let mut res = format!(
                        "You hit the {} with your {} for {} damage. It is dead.\n",
                        enemy_name, weapon, damage
                    );
                    if !enemy.loot().is_empty() {
                        res.push_str("It dropped:\n");
                        for x in enemy.loot().iter() {
                            res.push_str(&format!(" {},", x.1.name()));
                        }
                    }
                    CmdResult::new(true, res)
                }
            } else {
                CmdResult::new(false, format!("You do not have the \"{}\".", weapon))
            }
        } else {
            CmdResult::new(false, format!("There is no \"{}\" here.", enemy_name))
        }
    }

    // move an Item out of the current Room
    pub fn give(&mut self, name: &str) -> Option<Box<Item>> {
        self.get_curr_room_mut().items_mut().remove(name)
    }

    pub fn give_from(&mut self, item: &str, container: &str) -> Option<Box<Item>> {
        if let Some(cont) = self.get_curr_room_mut().items_mut().get_mut(container) {
            if let Some(ref mut contents) = cont.contents_mut() {
                contents.remove(item)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn give_all(&mut self) -> ItemMap {
        self.get_curr_room_mut().items_mut().drain().collect()
    }

    // insert an Item into the current Room
    pub fn insert(&mut self, name: &str, item: Option<Box<Item>>) -> CmdResult {
        if let Some(obj) = item {
            self.get_curr_room_mut()
                .items_mut()
                .insert(obj.name().to_string(), obj);
            CmdResult::new(true, "Dropped.".to_string())
        } else {
            CmdResult::new(false, format!("You do not have the \"{}\".", name))
        }
    }

    // insert an Item into a container Item in the current Room
    pub fn insert_into(
        &mut self,
        name: &str,
        container: &str,
        item: Option<Box<Item>>,
    ) -> CmdResult {
        if let Some(obj) = item {
            if let Some(cont) = self.get_curr_room_mut().items_mut().get_mut(container) {
                if let Some(ref mut contents) = cont.contents_mut() {
                    contents.insert(obj.name().to_string(), obj);
                    CmdResult::new(true, "Placed.".to_string())
                } else {
                    CmdResult::new(true, "You can not put anything in there.".to_string())
                }
            } else {
                CmdResult::new(false, format!("There is no \"{}\" here.", container))
            }
        } else {
            CmdResult::new(false, format!("You do not have the \"{}\".", name))
        }
    }
}
