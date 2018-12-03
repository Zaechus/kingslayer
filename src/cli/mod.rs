use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};

use item::Item;
use room::Room;
use utils::read_line::read_line;
use world::World;

/// A command line interface for controlling interactions between objects in a game
#[derive(Serialize, Deserialize)]
pub struct Cli {
    world: RefCell<World>,
    inventory: RefCell<HashMap<String, Box<Item>>>,
}

impl Cli {
    /// Cli constructor
    pub fn new(curr_room: &str, rooms: HashMap<String, Box<Room>>) -> Self {
        Self {
            world: RefCell::new(World::new(curr_room, rooms)),
            inventory: RefCell::new(HashMap::new()),
        }
    }
    /// starts the Cli session
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
                    "quit" | "q" => {
                        print!("Are you sure you want to quit? (y/N): ");
                        io::stdout().flush().expect("error flushing");
                        if read_line().starts_with('y') {
                            break;
                        }
                    }
                    _ => self.parse(&command),
                }
            } else {
                println!("I do not recognize that phrase.");
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
        let mut filtered: Vec<String> = words.to_vec();
        filtered.retain(|ref w| *w != "the" || *w != "a");
        filtered
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
            "take" | "grab" => {
                if words.len() > 1 {
                    self.take(&words[1..].join(" "));
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
            "put" | "place" => {
                if words.len() > 1 {
                    match words.iter().position(|r| r == "in") {
                        Some(loc) => {
                            self.put_in(&words[0..loc - 1].join(" "), &words[loc + 1..].join(" "))
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
    // take an Item from the current Room into the inventory
    fn take(&self, name: &str) {
        let curr_room = self.world.borrow().curr_room();
        let taken = match self.world.borrow_mut().rooms.get_mut(&curr_room) {
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
    // drop an Item from inventory into the current Room
    fn drop(&self, name: &str) {
        let curr_room = self.world.borrow().curr_room();
        let dropped = self.inventory.borrow_mut().remove(name);
        match dropped {
            Some(ob) => {
                if let Some(room) = self.world.borrow_mut().rooms.get_mut(&curr_room) {
                    room.items.insert(ob.name(), ob);
                    println!("Dropped.")
                }
            }
            None => println!("You don't have the \"{}\".", name),
        }
    }
    // place an Item into a container Item
    fn put_in(&self, item: &str, container: &str) {
        println!("TODO: put {} in {}", item, container);
    }
}

#[cfg(test)]
mod tests;
