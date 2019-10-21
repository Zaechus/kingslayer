use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{
    entity::{Closeable, Entity, Item, Room},
    types::{CmdResult, Items, RoomMap},
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
        if let Some(room) = self.rooms.get(&self.curr_room) {
            room
        } else {
            panic!(format!(
                "ERROR: {} is not a valid room (The world should be fixed).",
                self.curr_room
            ))
        }
    }

    pub fn get_curr_room_mut(&mut self) -> &mut Room {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            room
        } else {
            panic!(format!(
                "ERROR: {} is not a valid room (The world should be fixed).",
                self.curr_room
            ))
        }
    }

    // displays description of the current Room
    pub fn look(&self) -> CmdResult {
        CmdResult::new(true, self.get_curr_room().long_desc())
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        self.get_curr_room().inspect(name)
    }

    // changes the current Room to the target of the current Room's chosen path
    pub fn move_room(&mut self, direction: &str) -> CmdResult {
        if let Some(new_room) = self.get_curr_room().paths().get(direction) {
            if new_room.is_locked() == Some(true) {
                CmdResult::new(true, "The way is locked.".to_owned())
            } else if new_room.is_closed() {
                CmdResult::new(true, "The way is shut.".to_owned())
            } else {
                for enemy in self.get_curr_room().enemies() {
                    if enemy.is_angry() {
                        return CmdResult::new(false, "Enemies bar your way.".to_owned());
                    }
                }
                self.curr_room = new_room.name().to_owned();
                self.look()
            }
        } else {
            CmdResult::new(false, "You cannot go that way.".to_owned())
        }
    }

    pub fn open(&mut self, name: &str) -> CmdResult {
        self.get_curr_room_mut().open(name)
    }

    pub fn close(&mut self, name: &str) -> CmdResult {
        self.get_curr_room_mut().close(name)
    }

    pub fn clear_dead_enemies(&mut self) {
        self.get_curr_room_mut()
            .enemies_mut()
            .retain(|e| e.is_alive());
    }

    // let an Enemy in the current Room take damage
    pub fn harm_enemy(&mut self, damage: Option<i32>, enemy_name: &str, weapon: &str) -> CmdResult {
        if let Some(enemy) = self
            .get_curr_room()
            .enemies()
            .par_iter()
            .position_any(|item| item.name() == enemy_name)
        {
            if let Some(enemy) = self.get_curr_room_mut().enemies_mut().get_mut(enemy) {
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
                            for loot in enemy.loot() {
                                res.push_str(&format!(" {},", loot.long_name()));
                            }
                        }
                        CmdResult::new(true, res)
                    }
                } else {
                    CmdResult::dont_have(weapon)
                }
            } else {
                CmdResult::no_item_here(enemy_name)
            }
        } else {
            CmdResult::no_item_here(enemy_name)
        }
    }

    // move an Item out of the current Room
    pub fn give(&mut self, name: &str) -> Option<Box<Item>> {
        self.get_curr_room_mut().remove_item(name)
    }

    // take an Item from a container Item in the current Room
    pub fn give_from(
        &mut self,
        item_name: &str,
        container_name: &str,
    ) -> Result<Box<Item>, CmdResult> {
        self.get_curr_room_mut()
            .give_from(item_name, container_name)
    }

    pub fn give_all(&mut self) -> Items {
        self.get_curr_room_mut().drain_all()
    }

    // insert an Item into the current Room
    pub fn insert(&mut self, name: &str, item: Option<Box<Item>>) -> CmdResult {
        self.get_curr_room_mut().take_item(name, item)
    }

    // insert an Item into a container Item in the current Room
    pub fn insert_into(
        &mut self,
        item_name: &str,
        container_name: &str,
        item: Option<Box<Item>>,
    ) -> (CmdResult, Option<Box<Item>>) {
        self.get_curr_room_mut()
            .insert_into(item_name, container_name, item)
    }

    pub fn extend_items(&mut self, items: Items) {
        self.get_curr_room_mut().extend_items(items);
    }

    // interact with an Ally
    pub fn hail(&self, ally_name: &str) -> CmdResult {
        self.get_curr_room().hail(ally_name)
    }
}
