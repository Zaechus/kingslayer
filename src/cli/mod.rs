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
    world: RefCell<World>,
    inventory: RefCell<HashMap<String, Box<Item>>>,
}

impl Cli {
    pub fn new(curr_room: &str, rooms: HashMap<String, Box<Room>>) -> Self {
        Self {
            world: RefCell::new(World::new(curr_room, rooms)),
            inventory: RefCell::new(HashMap::new()),
        }
    }
    pub fn start(&self) {
        let mut player_name = String::new();
        while player_name.is_empty() {
            print!("Enter a character name: ");
            io::stdout().flush().expect("error flushing");
            player_name = read_line();
        }
        println!("Welcome, {}!\n", player_name);

        println!("{}", self.world.borrow().look());
        loop {
            let command = self.filter(&self.parts(&self.prompt()));
            if !command.is_empty() {
                // quit command
                match command.first().unwrap().as_str() {
                    "q" | "quit" => {
                        print!("Are you sure you want to quit? (y/N): ");
                        io::stdout().flush().expect("error flushing");
                        if read_line().starts_with('y') {
                            break;
                        }
                    }
                    _ => self.parse(&command),
                }
            } else {
                println!("I do not understand that phrase.");
            }
        }
        println!("Farewell, {}!", player_name);
    }
    // prompts the user for an action
    fn prompt(&self) -> String {
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
        words
            .to_vec()
            .into_iter()
            .filter(|w| w != "the" || w != "a" || w != "an")
            .collect()
    }
    // interprets words as game commands
    fn parse(&self, words: &[String]) {
        match words[0].as_str() {
            "l" | "look" => println!("{}", self.world.borrow().look()),
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                self.world.borrow_mut().move_room(&words[0])
            }
            "enter" => {
                if words.len() > 1 {
                    self.world.borrow_mut().move_room(&words[1])
                } else {
                    println!("Where do you want to {}?", words[0].as_str());
                }
            }
            "i" | "inventory" => println!("{}", self.inventory()),
            "take" | "get" => {
                if words.len() > 1 {
                    match words.iter().position(|r| r == "from") {
                        Some(pos) => {
                            self.take_from(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                        }
                        None => {
                            if words[1] == "all" {
                                self.take_all();
                            } else {
                                self.take(&words[1..].join(" "));
                            }
                        }
                    }
                } else {
                    println!("What do you want to {}?", words[0].as_str());
                }
            }
            "drop" => {
                if words.len() > 1 {
                    self.drop(&words[1..].join(" "));
                } else {
                    println!("What do you want to {}?", words[0].as_str());
                }
            }
            "examine" | "inspect" => {
                if words.len() > 1 {
                    println!("{}", self.inspect(&words[1..].join(" ")));
                } else {
                    println!("{} what?", &words[0])
                }
            }
            "put" | "place" => {
                if words.len() > 1 {
                    match words.iter().position(|r| r == "in") {
                        Some(pos) => {
                            self.put_in(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                        }
                        None => {
                            print!("{} ", &words[0]);
                            for word in &words[1..] {
                                print!("{} ", word);
                            }
                            println!("in what?");
                        }
                    }
                } else {
                    println!("{} what?", &words[0])
                }
            }
            _ => println!("I don't know the word \"{}\".", &words[0]),
        }
    }
    // prints inventory contents
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
    fn take(&self, name: &str) {
        let curr_room = &self.world.borrow().curr_room();
        let taken = match self.world.borrow_mut().rooms.get_mut(curr_room) {
            Some(room) => room.items.remove(name),
            None => None,
        };
        match taken {
            Some(ob) => {
                self.inventory.borrow_mut().insert(ob.name(), ob);
                println!("Taken.");
            }
            None => println!("There is no \"{}\" here.", name),
        }
    }
    // take all Items in the current Room
    fn take_all(&self) {
        let curr_room = &self.world.borrow().curr_room();
        if let Some(room) = self.world.borrow_mut().rooms.get_mut(curr_room) {
            for item in &room.items {
                self.inventory
                    .borrow_mut()
                    .insert(item.0.clone(), item.1.clone());
                println!("Taken.");
            }
            room.items.clear();
            room.items.shrink_to_fit();
        }
    }
    // take an item from a container in the room or inventory
    fn take_from(&self, item: &str, container: &str) {
        println!("TODO: take {} from {}", item, container);
    }
    // drop an Item from inventory into the current Room
    fn drop(&self, name: &str) {
        let curr_room = &self.world.borrow().curr_room();
        let dropped = self.inventory.borrow_mut().remove(name);
        match dropped {
            Some(obj) => {
                if let Some(room) = self.world.borrow_mut().rooms.get_mut(curr_room) {
                    room.items.insert(obj.name(), obj);
                    println!("Dropped.")
                }
            }
            None => println!("You don't have the \"{}\".", name),
        }
    }
    // place an Item into a container Item
    fn put_in(&self, item: &str, container: &str) {
        let curr_room = &self.world.borrow().curr_room();
        let placed = self.inventory.borrow_mut().remove(item);
        match placed {
            Some(obj) => {
                if let Some(room) = self.world.borrow_mut().rooms.get_mut(curr_room) {
                    match room.items.get_mut(container) {
                        Some(cont) => match cont.contents {
                            Some(ref mut contents) => {
                                contents.insert(obj.name(), obj);
                                println!("Placed.");
                            }
                            None => {
                                self.inventory.borrow_mut().insert(obj.name(), obj);
                                println!("You can't place anything in the {}.", container);
                            }
                        },
                        None => match self.inventory.borrow_mut().get_mut(container) {
                            Some(cont) => match cont.contents {
                                Some(ref mut contents) => {
                                    contents.insert(obj.name(), obj);
                                    println!("Placed.");
                                }
                                None => {
                                    self.inventory.borrow_mut().insert(obj.name(), obj);
                                    println!("You can't place anything in the {}.", container);
                                }
                            },
                            None => {
                                self.inventory.borrow_mut().insert(obj.name(), obj);
                                println!("There is no \"{}\" here.", container)
                            }
                        },
                    }
                }
            }
            None => println!("You don't have the \"{}\".", item),
        }
    }
}

#[cfg(test)]
mod tests;
