use serde_derive::{Deserialize, Serialize};

use crate::{
    entities::Item,
    types::{CmdResult, ItemMap, RoomMap, WorldError},
};

// Represents a world for the player to explore that consists of a grid of Rooms.
// A World is a graph data structure that encapsulates a collection of Room nodes.
#[derive(Serialize, Deserialize)]
pub struct World {
    curr_room: String,
    rooms: RoomMap,
}

impl World {
    pub fn curr_room(&self) -> String {
        self.curr_room.clone()
    }

    pub fn rooms_mut(&mut self) -> &mut RoomMap {
        &mut self.rooms
    }

    // displays description of the current Room
    pub fn look(&self) -> Result<CmdResult, WorldError> {
        if let Some(room) = self.rooms.get(&self.curr_room) {
            Ok(CmdResult::new(true, room.desc()))
        } else {
            Err(WorldError::NoRoom)
        }
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if let Some(room) = self.rooms.get(&self.curr_room) {
            if let Some(item) = room.items().get(name) {
                Some(CmdResult::new(true, item.inspection().to_string()))
            } else if let Some(item) = room.paths().get(name) {
                Some(CmdResult::new(true, item.inspection().to_string()))
            } else if let Some(enemy) = room.enemies().get(name) {
                Some(CmdResult::new(true, enemy.inspection().to_string()))
            } else {
                None
            }
        } else {
            None
        }
    }

    // changes the current Room to the target of the current Room's chosen path
    pub fn move_room(&mut self, direction: &str) -> Result<CmdResult, WorldError> {
        if let Some(room) = self.rooms.get(&self.curr_room) {
            if let Some(new_room) = room.paths().get(direction) {
                if new_room.is_locked() == Some(true) {
                    Ok(CmdResult::new(true, "The way is locked.".to_string()))
                } else if new_room.is_closed() == Some(true) {
                    Ok(CmdResult::new(true, "The way is closed.".to_string()))
                } else {
                    for e in room.enemies().values() {
                        if e.is_angry() {
                            return Ok(CmdResult::new(false, "Enemies bar your way.".to_string()));
                        }
                    }
                    self.curr_room = new_room.name().to_string();
                    self.look()
                }
            } else {
                Ok(CmdResult::new(false, "You cannot go that way.".to_string()))
            }
        } else {
            Err(WorldError::NoRoom)
        }
    }

    pub fn open_path(&mut self, path: &str) -> Result<CmdResult, WorldError> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(p) = room.paths_mut().get_mut(path) {
                if p.is_closed() == Some(true) {
                    p.open();
                    Ok(CmdResult::new(true, "Opened.".to_string()))
                } else {
                    Ok(CmdResult::new(
                        false,
                        format!("The {} is already opened.", path),
                    ))
                }
            } else {
                Ok(CmdResult::new(false, format!("There is no \"{}\".", path)))
            }
        } else {
            Err(WorldError::NoRoom)
        }
    }

    pub fn close_path(&mut self, path: &str) -> Result<CmdResult, WorldError> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(p) = room.paths_mut().get_mut(path) {
                if p.is_closed() == Some(true) {
                    Ok(CmdResult::new(
                        false,
                        format!("The {} is already closed.", path),
                    ))
                } else {
                    p.close();
                    Ok(CmdResult::new(true, "Closed.".to_string()))
                }
            } else {
                Ok(CmdResult::new(false, format!("There is no \"{}\".", path)))
            }
        } else {
            Err(WorldError::NoRoom)
        }
    }

    // let an Enemy in the current Room take damage
    pub fn harm_enemy(
        &mut self,
        damage: Option<i32>,
        enemy_name: &str,
        weapon: &str,
    ) -> Result<CmdResult, WorldError> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(enemy) = room.enemies_mut().get_mut(enemy_name) {
                if let Some(damage) = damage {
                    enemy.get_hit(damage);
                    if enemy.is_alive() {
                        Ok(CmdResult::new(
                            true,
                            format!(
                                "You hit the {} with your {} for {} damage.",
                                enemy_name, weapon, damage,
                            ),
                        ))
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
                        Ok(CmdResult::new(true, res))
                    }
                } else {
                    Ok(CmdResult::new(
                        false,
                        format!("You do not have the \"{}\".", weapon),
                    ))
                }
            } else {
                Ok(CmdResult::new(
                    false,
                    format!("There is no \"{}\" here.", enemy_name),
                ))
            }
        } else {
            Err(WorldError::NoRoom)
        }
    }

    // move an Item out of the current Room
    pub fn give(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            room.items_mut().remove(name)
        } else {
            None
        }
    }

    pub fn give_from(&mut self, item: &str, container: &str) -> Option<Box<Item>> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(cont) = room.items_mut().get_mut(container) {
                if let Some(ref mut contents) = cont.contents_mut() {
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

    pub fn give_all(&mut self) -> ItemMap {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            let items = room.items().clone();
            room.items_mut().clear();
            room.items_mut().shrink_to_fit();
            items
        } else {
            ItemMap::new()
        }
    }

    // insert an Item into the current Room
    pub fn insert(&mut self, name: &str, item: Option<Box<Item>>) -> Result<CmdResult, WorldError> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(obj) = item {
                room.items_mut().insert(obj.name().to_string(), obj);
                Ok(CmdResult::new(true, "Dropped.".to_string()))
            } else {
                Ok(CmdResult::new(
                    false,
                    format!("You do not have the \"{}\".", name),
                ))
            }
        } else {
            Err(WorldError::NoRoom)
        }
    }

    // insert an Item into a container Item in the current Room
    pub fn insert_into(
        &mut self,
        name: &str,
        container: &str,
        item: Option<Box<Item>>,
    ) -> Result<CmdResult, WorldError> {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            if let Some(obj) = item {
                if let Some(cont) = room.items_mut().get_mut(container) {
                    if let Some(ref mut contents) = cont.contents_mut() {
                        contents.insert(obj.name().to_string(), obj);
                        Ok(CmdResult::new(true, "Placed.".to_string()))
                    } else {
                        Ok(CmdResult::new(
                            true,
                            "You can not put anything in there.".to_string(),
                        ))
                    }
                } else {
                    Ok(CmdResult::new(
                        false,
                        format!("There is no \"{}\" here.", container),
                    ))
                }
            } else {
                Ok(CmdResult::new(
                    false,
                    format!("You do not have the \"{}\".", name),
                ))
            }
        } else {
            Err(WorldError::NoRoom)
        }
    }
}
