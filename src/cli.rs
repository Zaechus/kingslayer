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
    cmds: Vec<String>,
    verbs: Vec<String>,
    preps: Vec<String>,
    adjs: Vec<String>,
    nouns: Vec<String>,
}

impl Cli {
    /// Cli constructor
    pub fn new(rooms: Vec<Box<Room>>) -> Cli {
        Cli {
            world: RefCell::new(World::new(rooms)),
            inventory: RefCell::new(HashMap::new()),
            cmds: vec![
                "quit", "q", "look", "l", "i", "n", "s", "e", "w", "ne", "nw", "se", "sw", "u", "d",
            ].iter()
                .map(|x| x.to_string())
                .collect(),
            verbs: vec!["take", "drop", "put"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            preps: vec!["in"].iter().map(|x| x.to_string()).collect(),
            adjs: vec!["iron", "big", "red"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            nouns: vec!["sword", "block", "capsule"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        }
    }
    /// starts the Cli session
    pub fn start(&self) {
        let mut name = String::new();
        while name.is_empty() {
            print!("Enter a character name: ");
            io::stdout().flush().expect("error flushing");
            name = read_line();
        }
        println!("Welcome, {}!\n", name);

        self.world.borrow().look();
        loop {
            let command = self.filter(self.parts(self.prompt()));
            if !command.is_empty() {
                match command[0].as_str() {
                    "quit" | "q" => {
                        print!("Are you sure you want to quit? (y/N): ");
                        io::stdout().flush().expect("error flushing");
                        let response = read_line();
                        if !response.is_empty() && response.chars().next().unwrap() == 'y' {
                            break;
                        }
                    }
                    _ => self.parse(&command),
                }
            } else {
                println!("I do not recognize that phrase.");
            }
        }
        println!("Farewell, {}!", name);
    }
    /// prompts the user for a command
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
    /// splits a string into a vector of individual words
    fn parts(&self, s: String) -> Vec<String> {
        let out = s.split_whitespace().map(|x| x.to_lowercase().to_owned());
        let out: Vec<String> = out.collect();
        out
    }
    /// removes meaningless words
    fn filter(&self, words: Vec<String>) -> Vec<String> {
        let mut filtered: Vec<String> = Vec::new();
        for w in words.iter() {
            if self.cmds.contains(&w)
                || self.verbs.contains(&w)
                || self.preps.contains(&w)
                || self.adjs.contains(&w)
                || self.nouns.contains(&w)
            {
                filtered.push(w.clone());
            }
        }
        filtered
    }
    /// interprets words as game commands
    fn parse(&self, words: &Vec<String>) {
        if self.cmds.contains(&words[0]) {
            match words[0].as_str() {
                "l" | "look" => self.world.borrow().look(),
                "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                    self.world.borrow_mut().mv(&words[0])
                }
                "i" => self.inventory(),
                _ => println!("Nothing to do here."),
            }
        } else if words.len() > 1 {
            match words[0].as_str() {
                "take" => {
                    let item = self.gather_adj(&words);
                    self.take(item.clone());
                }
                "drop" => {
                    let item = self.gather_adj(&words);
                    self.drop(item.clone());
                }
                _ => (),
            }
        }
    }
    /// gathers adjectives followed by a noun from a slice into one string
    fn gather_adj(&self, words: &Vec<String>) -> String {
        if words.len() > 2 {
            let mut item = String::new();
            if self.nouns.contains(words.last().unwrap()) {
                for x in &words[1..words.len() - 1] {
                    if self.adjs.contains(x) {
                        item.push_str(&format!("{} ", x));
                    }
                }
                item.push_str(&words.last().unwrap());
            }
            item
        } else {
            words[1].clone()
        }
    }
    /// prints inventory contents
    fn inventory(&self) {
        if self.inventory.borrow().is_empty() {
            println!("You are empty-handed.");
        } else {
            println!("You are carrying:");
            for x in self.inventory.borrow().iter() {
                println!("  {}", x.1.name())
            }
        }
    }

    fn take(&self, name: String) {
        let curr_room = self.world.borrow().curr_room();
        // clone Item from World
        {
            let world = self.world.borrow_mut();
            match world.rooms[world.curr_room()].items.get(&name) {
                Some(ob) => {
                    self.inventory.borrow_mut().insert(
                        ob.name(),
                        Box::new(Item::new(&ob.name(), &ob.desc(), ob.is_container())),
                    );
                    println!("Taken.");
                }
                None => println!("There is no {}.", &name),
            }
        }
        // remove taken Item from World
        {
            let mut world = self.world.borrow_mut();
            world.rooms[curr_room].items.remove(&name);
        }
    }

    /// places an item into the current room
    fn drop(&self, name: String) {
        let curr_room = self.world.borrow().curr_room();
        match self.inventory.borrow().get(&name) {
            Some(ob) => {
                self.world.borrow_mut().rooms[curr_room].items.insert(
                    name.clone(),
                    Box::new(Item::new(&name, &ob.desc(), ob.is_container())),
                );
                println!("Dropped.");
            }
            None => println!("You don't have that."),
        }
        self.inventory.borrow_mut().remove(&name);
    }
}
