use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};

use item::Item;
use room::Room;
use utils::read_line;
use world::World;

/// A Command Line Interface struct;
/// controls all of the interactions between the user and all game objects
pub struct Cli {
    world: RefCell<World>,
    inventory: RefCell<HashMap<String, Box<Item>>>,
}

impl Cli {
    /// Cli constructor
    pub fn new(rooms: Vec<Box<Room>>) -> Self {
        Self {
            world: RefCell::new(World::new(rooms)),
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
            "exit" | "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                self.world.borrow_mut().move_room(&words[0])
            }
            "i" => println!("{}", self.inventory()),
            "enter" => self.world.borrow_mut().move_room(&words[1]),
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
            _ => println!("I don't know the word \"{}\".", &words[0]),
        }
    }
    // prints inventory contents
    fn inventory(&self) -> String {
        if self.inventory.borrow().is_empty() {
            "You are empty-handed.".to_owned()
        } else {
            let mut inv = String::from("You are carrying:\n");
            for x in self.inventory.borrow().iter() {
                inv = format!("{}  {}\n", inv, x.1.name())
            }
            inv
        }
    }
    // take an Obj from the current Room into the inventory
    fn take(&self, name: &str) {
        let curr_room = self.world.borrow().curr_room();
        let taken = self.world.borrow_mut().rooms[curr_room].items.remove(name);
        match taken {
            Some(ob) => {
                self.inventory.borrow_mut().insert(ob.name(), ob);
                println!("Taken.");
            }
            None => println!("There is no {} here.", name),
        }
    }
    // drop an Obj from inventory into the current Room
    fn drop(&self, name: &str) {
        let curr_room = self.world.borrow().curr_room();
        let dropped = self.inventory.borrow_mut().remove(name);
        match dropped {
            Some(ob) => {
                self.world.borrow_mut().rooms[curr_room]
                    .items
                    .insert(ob.name(), ob);
                println!("Dropped.");
            }
            None => println!("You don't have that."),
        }
    }
}

#[cfg(test)]
mod tests;
