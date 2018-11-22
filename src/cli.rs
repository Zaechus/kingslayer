// Copyright (c) 2018 Maxwell Anderson

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};

use items::obj::Obj;
use room::Room;
use utils::read_line;
use world::World;

/// A Command Line Interface struct;
/// controls all of the interactions between the user and all game objects
pub struct Cli {
    world: RefCell<World>,
    inventory: RefCell<HashMap<String, Box<Obj>>>,
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
                "quit", "q", "look", "l", "i", "exit", "n", "s", "e", "w", "ne", "nw", "se", "sw",
                "u", "d",
            ].iter()
            .map(|x| x.to_string())
            .collect(),
            verbs: vec!["enter", "take", "drop", "put", "place"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            preps: vec!["in"].iter().map(|x| x.to_string()).collect(),
            adjs: vec!["iron", "big", "red"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            nouns: vec!["sword", "block", "capsule", "closet"]
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

        println!("{}", self.world.borrow().look());
        loop {
            let command = self.filter(&self.parts(&self.prompt()));
            if !command.is_empty() {
                match command[0].as_str() {
                    "quit" | "q" => {
                        print!("Are you sure you want to quit? (y/N): ");
                        io::stdout().flush().expect("error flushing");
                        let response = read_line();
                        if !response.is_empty() && response.starts_with('y') {
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
    // prompts the user for a command
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
        let out = s.split_whitespace().map(|x| x.to_lowercase().to_owned());
        let out: Vec<String> = out.collect();
        out
    }
    // removes meaningless words
    fn filter(&self, words: &[String]) -> Vec<String> {
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
    // interprets words as game commands
    fn parse(&self, words: &[String]) {
        if self.cmds.contains(&words[0]) {
            match words[0].as_str() {
                "l" | "look" => println!("{}", self.world.borrow().look()),
                "exit" | "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                    self.world.borrow_mut().mv(&words[0])
                }
                "i" => println!("{}", self.inventory()),
                _ => println!("Nothing to do here."),
            }
        } else if words.len() > 1 {
            match words[0].as_str() {
                "enter" => self.world.borrow_mut().mv(&words[1]),
                "take" => {
                    let item = self.gather_adj(&words[1..]);
                    self.take(&item);
                }
                "drop" => {
                    let item = self.gather_adj(&words[1..]);
                    self.drop(&item);
                }
                "put" | "place" => {
                    // TODO
                    if words.contains(&"in".to_string()) {
                        let in_index = words.iter().position(|r| r == &"in".to_string()).unwrap();
                        let item = self.gather_adj(&words[1..in_index]);
                        let container = self.gather_adj(&words[in_index + 1..words.len()]);
                        self.put_in(&item, &container);
                    } else {
                        println!("Put it in what?");
                    }
                }
                _ => println!("That doesn't make any sense."),
            }
        }
    }

    fn gather_adj(&self, words: &[String]) -> String {
        if words.len() > 1 {
            let mut item = String::new();
            if self.nouns.contains(words.last().unwrap()) {
                for x in &words[0..words.len() - 1] {
                    if self.adjs.contains(x) {
                        item.push_str(&format!("{} ", x));
                    }
                }
                item.push_str(&words.last().unwrap());
            }
            item
        } else {
            words.first().unwrap().clone()
        }
    }

    // prints inventory contents
    fn inventory(&self) -> String {
        if self.inventory.borrow().is_empty() {
            "You are empty-handed.".to_string()
        } else {
            let mut inv = String::from("You are carrying:\n");
            for x in self.inventory.borrow().iter() {
                inv = format!("{}  {}\n", inv, x.1.name())
            }
            inv
        }
    }

    fn take(&self, name: &str) {
        let curr_room = self.world.borrow().curr_room();
        let taken = self.world.borrow_mut().rooms[curr_room].items.remove(name);
        match taken {
            Some(ob) => {
                self.inventory.borrow_mut().insert(ob.name(), ob);
                println!("Taken.");
            }
            None => println!("There is no {} here.", &name),
        }
    }

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

    // places an Obj into a Container in the currrent room
    fn put_in(&self, item: &str, _container: &str) {
        // TODO
        let _curr_room = self.world.borrow().curr_room();
        match self.inventory.borrow().get(item) {
            Some(_ob) => {}
            None => println!("You don't have that."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use items::item::Item;

    #[test]
    fn cli_take_drop() {
        let iron_sword = Box::new(Item::new(
            "iron sword",
            "There is an iron sword on the ground.",
        ));
        let capsule = Box::new(Item::new("capsule", "There is a capsule here."));

        let mut sandbox_room_objs: HashMap<String, Box<Obj>> = HashMap::new();
        sandbox_room_objs.insert(iron_sword.name(), iron_sword);
        sandbox_room_objs.insert(capsule.name(), capsule);

        let sandbox_room = Box::new(Room::new(
            "Sandbox Room",
            "You stand in a large box filled with sand.",
            sandbox_room_objs,
        ));
        let rooms: Vec<Box<Room>> = vec![sandbox_room];

        let cli = Cli::new(rooms);

        assert_eq!(cli.inventory(), "You are empty-handed.");
        assert!(
            cli.world.borrow().look().contains("iron sword")
                && cli.world.borrow().look().contains("capsule")
        );

        cli.take("iron sword");
        assert_eq!(cli.inventory(), "You are carrying:\n  iron sword\n");
        assert_eq!(
            cli.world.borrow().look(),
            format!(
                "{}{}{}",
                "Sandbox Room\n",
                "You stand in a large box filled with sand.\n",
                "There is a capsule here."
            )
        );

        cli.take("capsule");
        assert!(cli.inventory().contains("iron sword") && cli.inventory().contains("capsule"));
        assert_eq!(
            cli.world.borrow().look(),
            format!(
                "{}{}",
                "Sandbox Room\n", "You stand in a large box filled with sand.",
            )
        );

        cli.drop("iron sword");
        assert_eq!(cli.inventory(), "You are carrying:\n  capsule\n");
        assert_eq!(
            cli.world.borrow().look(),
            format!(
                "{}{}{}",
                "Sandbox Room\n",
                "You stand in a large box filled with sand.\n",
                "There is an iron sword on the ground.",
            )
        );

        cli.drop("capsule");
        assert_eq!(cli.inventory(), "You are empty-handed.");
        assert!(
            cli.world.borrow().look().contains("iron sword")
                && cli.world.borrow().look().contains("capsule")
        );
    }
}
