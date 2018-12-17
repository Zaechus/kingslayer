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
    hp: i32,
    world: RefCell<World>,
    inventory: RefCell<HashMap<String, Box<Item>>>,
}

impl Cli {
    pub fn new(curr_room: &str, rooms: HashMap<String, Box<Room>>) -> Self {
        Self {
            hp: 100,
            world: RefCell::new(World::new(curr_room, rooms)),
            inventory: RefCell::new(HashMap::new()),
        }
    }

    pub fn ask(&self, input: &str) -> String {
        let command = self.mod_directions(&self.filter(&self.parts(input)));
        if !command.is_empty() {
            self.parse(&command)
        } else {
            "I do not understand that phrase.".to_owned()
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
            .map(|x| x.to_lowercase().to_owned())
            .collect()
    }

    // removes meaningless words
    fn filter(&self, words: &[String]) -> Vec<String> {
        let mut filtered = Vec::with_capacity(words.len());
        for w in words {
            match w.as_str() {
                "the" | "a" | "an" | "go" => (),
                _ => filtered.push(w.to_owned()),
            }
        }
        filtered
    }

    // modify path directives
    fn mod_directions(&self, words: &[String]) -> Vec<String> {
        let mut modified = Vec::with_capacity(words.len());
        for w in words {
            match w.as_str() {
                "north" => modified.push("n".to_owned()),
                "south" => modified.push("s".to_owned()),
                "east" => modified.push("e".to_owned()),
                "west" => modified.push("w".to_owned()),
                "northeast" => modified.push("ne".to_owned()),
                "northwest" => modified.push("nw".to_owned()),
                "southeast" => modified.push("se".to_owned()),
                "southwest" => modified.push("sw".to_owned()),
                "up" => modified.push("u".to_owned()),
                "down" => modified.push("d".to_owned()),
                _ => modified.push(w.to_owned()),
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
                    format!("Where do you want to {}?", words[0].as_str())
                }
            }
            "i" | "inventory" => self.inventory(),
            "take" | "get" => {
                if words.len() > 1 {
                    match words.iter().position(|r| r == "from") {
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
                    format!("What do you want to {}?", words[0].as_str())
                }
            }
            "drop" => {
                if words.len() > 1 {
                    self.drop(&words[1..].join(" "))
                } else {
                    format!("What do you want to {}?", words[0].as_str())
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
            _ => format!("I don't know the word \"{}\".", &words[0]),
        }
    }

    // returns inventory contents
    fn inventory(&self) -> String {
        if self.inventory.borrow().is_empty() {
            "You are empty-handed.".to_owned()
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
        format!("You have {} HP.", self.hp)
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
            None => "You are not in a room...".to_owned(),
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
                "Taken.".to_owned()
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
        taken_str
    }

    // take an item from a container in the room or inventory
    fn take_from(&self, item: &str, container: &str) -> String {
        format!("TODO: take {} from {}", item, container)
    }

    // drop an Item from inventory into the current Room
    fn drop(&self, name: &str) -> String {
        let curr_room = &self.world.borrow().curr_room();
        let dropped = self.inventory.borrow_mut().remove(name);
        match dropped {
            Some(obj) => {
                if let Some(room) = self.world.borrow_mut().rooms.get_mut(curr_room) {
                    room.items.insert(obj.name(), obj);
                    "Dropped.".to_owned()
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
                                "Placed.".to_owned()
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
                                    "Placed.".to_owned()
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
