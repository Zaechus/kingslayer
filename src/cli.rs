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
            verbs: vec!["take", "put"].iter().map(|x| x.to_string()).collect(),
            preps: vec!["in"].iter().map(|x| x.to_string()).collect(),
            adjs: vec!["iron", "big", "red"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            nouns: vec!["sword", "block"]
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
            let command = self.prompt();
            let command = self.parts(command);
            let command = self.filter(command);
            if !command.is_empty() {
                match command[0].as_str() {
                    "quit" | "q" => break,
                    _ => (),
                }
                self.parse(&command);
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
            self.commands(&words[0]);
        } else if words.len() > 1 {
            if words[0] == "take" {
                let mut item = String::new();
                if self.nouns.contains(words.last().unwrap()) {}
                if words.len() > 2 {
                    for x in &words[1..words.len() - 1] {
                        if self.adjs.contains(x) {
                            item.push_str(&format!("{} ", x));
                        }
                    }
                }
                item.push_str(&words.last().unwrap());
                self.take(item.clone());
                let curr_room = self.world.borrow().curr_room();
                self.remove_item(item, curr_room);
            }
        }
    }
    fn commands(&self, cmd: &str) {
        match cmd {
            "l" | "look" => self.world.borrow().look(),
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                self.world.borrow_mut().mv(cmd)
            }
            "i" => self.inventory(),
            _ => println!("Nothing to do here."),
        }
    }
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
        let world = self.world.borrow_mut();
        match world.rooms[world.curr_room()].items.get(&name) {
            Some(ob) => {
                self.inventory
                    .borrow_mut()
                    .insert(ob.desc(), Box::new(Item::new(&ob.name(), &ob.desc())));
                println!("Taken.");
            }
            None => println!("There is no {}.", &name),
        }
    }
    fn remove_item(&self, name: String, curr_room: usize) {
        let mut world = self.world.borrow_mut();
        world.rooms[curr_room].items.remove(&name);
    }
}
