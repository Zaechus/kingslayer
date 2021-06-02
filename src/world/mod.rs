use serde::{Deserialize, Serialize};

use crate::{
    entity::{Closeable, Element, Enemy, Entity, Item, Lockable, Room},
    types::{Action, Attack, CmdResult, Items, Rooms},
};

// Represents a world for the player to explore that consists of a grid of Rooms.
// A World is a graph data structure that encapsulates a collection of Room nodes.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct World {
    curr_room: String,
    rooms: Rooms,
}

impl World {
    pub fn get_curr_room(&self) -> &Room {
        if let Some(room) = self.rooms.get(&self.curr_room) {
            room
        } else {
            panic!(
                "ERROR: {} is not a valid room (The world should be fixed).",
                self.curr_room
            )
        }
    }

    pub fn get_curr_room_mut(&mut self) -> &mut Room {
        if let Some(room) = self.rooms.get_mut(&self.curr_room) {
            room
        } else {
            panic!(
                "ERROR: {} is not a valid room (The world should be fixed).",
                self.curr_room
            )
        }
    }

    fn get_room_mut(&mut self, room_name: &str) -> &mut Room {
        if let Some(room) = self.rooms.get_mut(room_name) {
            room
        } else {
            panic!(
                "ERROR: {} is not a valid room (The world should be fixed).",
                room_name
            )
        }
    }

    // displays description of the current Room
    pub fn look(&self) -> CmdResult {
        CmdResult::new(Action::Active, self.get_curr_room().long_desc())
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        self.get_curr_room().inspect(name)
    }

    pub fn any_angry_enemies(&self) -> bool {
        for enemy in self.get_curr_room().enemies() {
            if enemy.is_angry() {
                return true;
            }
        }
        false
    }

    // changes the current Room to the target of the current Room's chosen path
    pub fn move_room(&mut self, direction: &str) -> CmdResult {
        if let Some(path) = self.get_curr_room().find_path(direction) {
            if path.is_closed() {
                CmdResult::new(Action::Active, "The way is shut.")
            } else if path.is_locked() {
                CmdResult::is_locked(direction)
            } else if self.any_angry_enemies() {
                CmdResult::new(Action::Passive, "Enemies bar your way.")
            } else {
                self.curr_room = path.name().to_owned();
                self.look()
            }
        } else {
            CmdResult::new(Action::Passive, "You cannot go that way.")
        }
    }

    pub fn unlock(&mut self, name: &str) -> CmdResult {
        self.get_curr_room_mut().unlock(name)
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

    // have an Enemy in the current Room take damage
    pub fn harm_enemy(&mut self, enemy_name: &str, attack: Attack) -> CmdResult {
        self.get_curr_room_mut().harm_enemy(enemy_name, attack)
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

    // interact with an Ally
    pub fn hail(&self, ally_name: &str) -> CmdResult {
        self.get_curr_room().hail(ally_name)
    }

    pub fn add_element(&mut self, room: &str, el: Element) {
        self.get_room_mut(room).add_element(el);
    }

    pub fn add_item(&mut self, room: &str, item: Item) {
        self.get_room_mut(room).add_item(item);
    }

    pub fn spawn_enemy(&mut self, room: &str, enemy: Enemy) {
        self.get_room_mut(room).spawn_enemy(enemy);
    }
}
