use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::item::Item;
use crate::room::Room;
use crate::utils::read_line::read_line;
use crate::world::World;

// A command line interface for controlling interactions between objects in a game
#[derive(Serialize, Deserialize)]
pub struct Cli {
    hp: RefCell<(i32, i32)>,
    world: RefCell<World>,
    inventory: RefCell<HashMap<String, Box<Item>>>,
}

impl Cli {
    pub fn new(curr_room: &str, rooms: HashMap<String, Box<Room>>) -> Self {
        Self {
            hp: RefCell::new((10, 10)),
            world: RefCell::new(World::new(curr_room, rooms)),
            inventory: RefCell::new(HashMap::new()),
        }
    }

    pub fn ask(&self, input: &str) -> String {
        let command = self.mod_directions(&self.filter(&self.parts(input)));
        if !command.is_empty() {
            format!("{}{}", self.parse(&command), self.events())
        } else {
            "I do not understand that phrase.".to_string()
        }
    }

    // prompts the user for an action
    pub fn prompt(&self) -> String {
        loop {
            print!("\n> ");
            io::stdout().flush().expect("error flushing");
            let input = read_line();
            if !input.is_empty() {
                return input;
            } else {
                println!("Excuse me?");
            }
        }
    }

    // splits a string into a vector of individual words
    fn parts(&self, s: &str) -> Vec<String> {
        s.split_whitespace()
            .map(|x| x.to_lowercase().to_string())
            .collect()
    }

    // removes meaningless words
    fn filter(&self, words: &[String]) -> Vec<String> {
        let mut filtered = Vec::with_capacity(words.len());
        for w in words {
            match w.as_str() {
                "the" | "a" | "an" | "go" | "of" => (),
                _ => filtered.push(w.to_string()),
            }
        }
        filtered
    }

    // modify path directives
    fn mod_directions(&self, words: &[String]) -> Vec<String> {
        let mut modified = Vec::with_capacity(words.len());
        for w in words {
            match w.as_str() {
                "north" => modified.push("n".to_string()),
                "south" => modified.push("s".to_string()),
                "east" => modified.push("e".to_string()),
                "west" => modified.push("w".to_string()),
                "northeast" => modified.push("ne".to_string()),
                "northwest" => modified.push("nw".to_string()),
                "southeast" => modified.push("se".to_string()),
                "southwest" => modified.push("sw".to_string()),
                "up" => modified.push("u".to_string()),
                "down" => modified.push("d".to_string()),
                _ => modified.push(w.to_string()),
            }
        }
        modified
    }

    // interprets words as game commands
    fn parse(&self, words: &[String]) -> String {
        match words[0].as_str() {
            "l" | "look" => self.world.borrow().look(),
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                self.world.borrow_mut().move_room(&words[0])
            }
            "enter" => {
                if words.len() > 1 {
                    self.world.borrow_mut().move_room(&words[1])
                } else {
                    format!("Where do you want to {}?", words[0])
                }
            }
            "i" | "inventory" => self.inventory(),
            "take" | "get" => {
                if words.len() > 1 {
                    match words.iter().position(|r| r == "from" || r == "out") {
                        Some(pos) => {
                            self.take_from(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                        }
                        None => {
                            if words[1] == "all" {
                                self.take_all()
                            } else {
                                self.take(&words[1..].join(" "))
                            }
                        }
                    }
                } else {
                    format!("What do you want to {}?", words[0])
                }
            }
            "drop" => {
                if words.len() > 1 {
                    self.drop(&words[1..].join(" "))
                } else {
                    format!("What do you want to {}?", words[0])
                }
            }
            "examine" | "inspect" | "read" => {
                if words.len() > 1 {
                    self.inspect(&words[1..].join(" "))
                } else {
                    format!("{} what?", &words[0])
                }
            }
            "status" | "diagnostic" => self.status(),
            "put" | "place" => {
                if words.len() > 1 {
                    match words.iter().position(|r| r == "in") {
                        Some(pos) => {
                            self.put_in(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                        }
                        None => format!("{} in what?", words.join(" ")),
                    }
                } else {
                    format!("{} what?", &words[0])
                }
            }
            "attack" | "slay" | "kill" | "hit" => {
                if words.len() > 1 {
                    match words.iter().position(|r| r == "with") {
                        Some(pos) => {
                            self.attack(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                        }
                        None => format!("{} with what?", &words[0]),
                    }
                } else {
                    format!("What do you want to {}?", &words[0])
                }
            }
            _ => format!("I don't know the word \"{}\".", &words[0]),
        }
    }

    // computes actions taken by Enemies in the current room
    fn events(&self) -> String {
        let curr_room = &self.world.borrow().curr_room();
        let mut hp = *self.hp.borrow();
        match self.world.borrow_mut().rooms.get_mut(curr_room) {
            Some(room) => {
                let mut events_str = String::new();
                for e in room.enemies.iter() {
                    let e_dmg = e.1.damage();
                    self.hp.replace((hp.0 - e_dmg, hp.1));
                    hp = *self.hp.borrow();
                    events_str.push_str(&format!(
                        "\nThe {} hit you for {} damage. You have {} HP left.",
                        e.1.name(),
                        e_dmg,
                        hp.0
                    ));
                }
                room.enemies.retain(|_, e| e.hp() > 0);
                if hp.0 <= 0 {
                    events_str.push_str("You died.");
                }
                events_str
            }
            None => "You are not in a room...".to_string(),
        }
    }

    fn attack(&self, enemy: &str, weapon: &str) -> String {
        let curr_room = &self.world.borrow().curr_room();
        match self.world.borrow_mut().rooms.get_mut(curr_room) {
            Some(room) => match room.enemies.get_mut(enemy) {
                Some(nme) => match self.inventory.borrow().get(weapon) {
                    Some(wpon) => {
                        let dmg = wpon.damage();
                        nme.get_hit(dmg);
                        if nme.hp() > 0 {
                            format!(
                                "You hit the {} with your {} for {} damage. It has {} HP left.",
                                enemy,
                                weapon,
                                dmg,
                                nme.hp()
                            )
                        } else {
                            format!(
                                "You hit the {} with your {} for {} damage. It is dead.",
                                enemy, weapon, dmg
                            )
                        }
                    }
                    None => format!("You do not have the {}.", weapon),
                },
                None => format!("There is no {} here.", enemy),
            },
            None => "You are not in a room...".to_string(),
        }
    }

    // returns inventory contents
    fn inventory(&self) -> String {
        if self.inventory.borrow().is_empty() {
            "You are empty-handed.".to_string()
        } else {
            let mut items_carried = String::from("You are carrying:");
            for x in self.inventory.borrow().iter() {
                items_carried = format!("{}\n  {}", items_carried, x.1.name());
            }
            items_carried
        }
    }

    // returns HP
    fn status(&self) -> String {
        format!(
            "You have ({} / {}) HP.",
            self.hp.borrow().0,
            self.hp.borrow().1
        )
    }

    // returns the special properties of an object or path
    pub fn inspect(&self, name: &str) -> String {
        let curr_room = &self.world.borrow().curr_room();
        match self.world.borrow().rooms.get(curr_room) {
            Some(room) => match room.items.get(name) {
                Some(item) => item.inspection(),
                None => match self.inventory.borrow().get(name) {
                    Some(item) => item.inspection(),
                    None => match room.paths.get(name) {
                        Some(item) => item.inspection(),
                        None => format!("There is no \"{}\" here.", name),
                    },
                },
            },
            None => "You are not in a room...".to_string(),
        }
    }

    // take an Item from the current Room into the inventory
    fn take(&self, name: &str) -> String {
        let curr_room = &self.world.borrow().curr_room();
        let taken = match self.world.borrow_mut().rooms.get_mut(curr_room) {
            Some(room) => room.items.remove(name),
            None => None,
        };
        match taken {
            Some(ob) => {
                self.inventory.borrow_mut().insert(ob.name(), ob);
                "Taken.".to_string()
            }
            None => format!("There is no \"{}\" here.", name),
        }
    }

    // take all Items in the current Room
    fn take_all(&self) -> String {
        let curr_room = &self.world.borrow().curr_room();
        let mut taken_str = String::new();
        if let Some(room) = self.world.borrow_mut().rooms.get_mut(curr_room) {
            for item in &room.items {
                self.inventory
                    .borrow_mut()
                    .insert(item.0.clone(), item.1.clone());
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
    fn take_from(&self, item: &str, container: &str) -> String {
        let curr_room = &self.world.borrow().curr_room();
        let taken = match self.world.borrow_mut().rooms.get_mut(curr_room) {
            Some(room) => match room.items.get_mut(container) {
                Some(cont) => match cont.contents {
                    Some(ref mut contents) => contents.remove(item),
                    None => None,
                },
                None => match self.inventory.borrow_mut().get_mut(container) {
                    Some(cont2) => match cont2.contents {
                        Some(ref mut contents) => contents.remove(item),
                        None => None,
                    },
                    None => return format!("There is no {} here.", container),
                },
            },
            None => None,
        };
        match taken {
            Some(ob) => {
                self.inventory.borrow_mut().insert(ob.name(), ob);
                "Taken.".to_string()
            }
            None => format!("There is no \"{}\" here.", item),
        }
    }

    // drop an Item from inventory into the current Room
    fn drop(&self, name: &str) -> String {
        let curr_room = &self.world.borrow().curr_room();
        let dropped = self.inventory.borrow_mut().remove(name);
        match dropped {
            Some(obj) => {
                if let Some(room) = self.world.borrow_mut().rooms.get_mut(curr_room) {
                    room.items.insert(obj.name(), obj);
                    "Dropped.".to_string()
                } else {
                    String::new()
                }
            }
            None => format!("You don't have the \"{}\".", name),
        }
    }

    // place an Item into a container Item
    fn put_in(&self, item: &str, container: &str) -> String {
        let curr_room = &self.world.borrow().curr_room();
        let placed = self.inventory.borrow_mut().remove(item);
        match placed {
            Some(obj) => {
                if let Some(room) = self.world.borrow_mut().rooms.get_mut(curr_room) {
                    match room.items.get_mut(container) {
                        Some(cont) => match cont.contents {
                            Some(ref mut contents) => {
                                contents.insert(obj.name(), obj);
                                "Placed.".to_string()
                            }
                            None => {
                                self.inventory.borrow_mut().insert(obj.name(), obj);
                                format!("You can't place anything in the {}.", container)
                            }
                        },
                        None => match self.inventory.borrow_mut().get_mut(container) {
                            Some(cont) => match cont.contents {
                                Some(ref mut contents) => {
                                    contents.insert(obj.name(), obj);
                                    "Placed.".to_string()
                                }
                                None => {
                                    self.inventory.borrow_mut().insert(obj.name(), obj);
                                    format!("You can't place anything in the {}.", container)
                                }
                            },
                            None => {
                                self.inventory.borrow_mut().insert(obj.name(), obj);
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
}

#[cfg(test)]
mod tests;
