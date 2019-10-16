use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{
    entity::{
        Closeable, Entity,
        Item::{self, Container},
        Room,
    },
    player::Player,
    response::{dont_have, no_item_here, not_container},
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
            panic!("ERROR: You are not in a valid room (The world should be fixed).")
        }
    }

    pub fn get_curr_room_mut(&mut self) -> &mut Room {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            room
        } else {
            panic!("ERROR: You are not in a valid room (The world should be fixed).")
        }
    }

    // displays description of the current Room
    pub fn look(&self) -> CmdResult {
        CmdResult::new(true, self.get_curr_room().long_desc())
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if let Some(item) = self
            .get_curr_room()
            .items()
            .par_iter()
            .find_any(|x| x.name() == name)
        {
            Some(CmdResult::new(true, item.inspect().to_owned()))
        } else if let Some(pathway) = self.get_curr_room().paths().get(name) {
            Some(CmdResult::new(true, pathway.inspect().to_owned()))
        } else if let Some(enemy) = self.get_curr_room().enemies().get(name) {
            Some(CmdResult::new(true, enemy.inspect().to_owned()))
        } else if let Some(ally) = self.get_curr_room().allies().get(name) {
            Some(CmdResult::new(true, ally.inspect().to_owned()))
        } else {
            None
        }
    }

    // changes the current Room to the target of the current Room's chosen path
    pub fn move_room(&mut self, direction: &str) -> CmdResult {
        if let Some(new_room) = self.get_curr_room().paths().get(direction) {
            if new_room.is_locked() == Some(true) {
                CmdResult::new(true, "The way is locked.".to_owned())
            } else if new_room.is_closed() {
                CmdResult::new(true, "The way is shut.".to_owned())
            } else {
                for enemy in self.get_curr_room().enemies().values() {
                    if enemy.is_angry() {
                        return CmdResult::new(false, "Enemies bar your way.".to_owned());
                    }
                }
                self.curr_room = new_room.target().to_owned();
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
                        for loot in enemy.loot() {
                            res.push_str(&format!(" {},", loot.long_name()));
                        }
                    }
                    CmdResult::new(true, res)
                }
            } else {
                dont_have(weapon)
            }
        } else {
            no_item_here(enemy_name)
        }
    }

    // move an Item out of the current Room
    pub fn give(&mut self, name: &str) -> Option<Box<Item>> {
        self.get_curr_room_mut().remove_item(name)
    }

    // take an Item from a container Item in the current Room
    pub fn give_from(
        &mut self,
        player: &mut Player,
        item_name: &str,
        container_name: &str,
    ) -> CmdResult {
        if let Some(container) = self
            .get_curr_room_mut()
            .items_mut()
            .par_iter_mut()
            .find_any(|x| x.name() == container_name)
        {
            if let Container(container) = &mut **container {
                if container.is_closed() {
                    CmdResult::new(true, format!("The {} is closed.", container_name))
                } else if let Some(item) = container
                    .contents_mut()
                    .par_iter()
                    .position_any(|x| x.name() == item_name)
                {
                    player.take(item_name, Some(container.remove(item)));
                    CmdResult::new(true, "Taken.".to_owned())
                } else {
                    CmdResult::new(
                        false,
                        format!("There is no {} in the {}.", item_name, container_name),
                    )
                }
            } else {
                not_container(container_name)
            }
        } else {
            no_item_here(container_name)
        }
    }

    pub fn give_all(&mut self) -> Items {
        self.get_curr_room_mut().items_mut().drain(0..).collect()
    }

    // insert an Item into the current Room
    pub fn insert(&mut self, name: &str, item: Option<Box<Item>>) -> CmdResult {
        if let Some(obj) = item {
            self.get_curr_room_mut().items_mut().push(obj);
            CmdResult::new(true, "Dropped.".to_owned())
        } else {
            dont_have(name)
        }
    }

    // insert an Item into a container Item in the current Room
    pub fn insert_into(
        &mut self,
        player: &mut Player,
        item_name: &str,
        container_name: &str,
    ) -> CmdResult {
        if let Some(item) = player.remove(item_name) {
            if let Some(container) = self
                .get_curr_room_mut()
                .items_mut()
                .par_iter_mut()
                .find_any(|x| x.name() == container_name)
            {
                if let Container(container) = &mut **container {
                    if container.is_closed() {
                        player.take(item_name, Some(item));
                        CmdResult::new(true, format!("The {} is closed.", container_name))
                    } else {
                        container.push(item);
                        CmdResult::new(true, "Placed.".to_owned())
                    }
                } else {
                    player.take(item_name, Some(item));
                    not_container(container_name)
                }
            } else {
                player.take(item_name, Some(item));
                no_item_here(container_name)
            }
        } else {
            dont_have(item_name)
        }
    }

    // interact with an Ally
    pub fn hail(&self, ally_name: &str) -> CmdResult {
        if let Some(_ally) = self.get_curr_room().allies().get(ally_name) {
            CmdResult::new(false, "TODO: interact with ally".to_owned())
        } else {
            no_item_here(ally_name)
        }
    }
}
