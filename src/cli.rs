use std::cell::RefCell;
use std::io;
use std::io::Write;

use obj::Obj;
use room::Room;
use utils::read_line;
use world::World;

/// A Command Line Interface struct;
/// controls all of the interactions between the user and all game objects
pub struct Cli {
    world: RefCell<World>,
    inventory: Vec<Box<Obj>>,
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
            inventory: Vec::new(),
            cmds: vec![
                "quit", "q", "look", "l", "i", "n", "s", "e", "w", "ne", "nw", "se", "sw",
            ].iter()
                .map(|x| x.to_string())
                .collect(),
            verbs: vec!["asdf"].iter().map(|x| x.to_string()).collect(),
            preps: vec!["asdf"].iter().map(|x| x.to_string()).collect(),
            adjs: vec!["asdf"].iter().map(|x| x.to_string()).collect(),
            nouns: vec!["asdf"].iter().map(|x| x.to_string()).collect(),
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
            self.actions(&words[0]);
        }
    }
    fn actions(&self, cmd: &str) {
        match cmd {
            "l" | "look" => self.world.borrow().look(),
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" => self.world.borrow_mut().mv(cmd),
            "i" => self.inventory(),
            _ => println!("Nothing to do here."),
        }
    }
    fn inventory(&self) {
        if self.inventory.is_empty() {
            println!("You are empty-handed.");
        } else {
            println!("You are carrying:");
            for x in self.inventory.iter() {
                println!("  {}", x.desc())
            }
        }
    }
}
