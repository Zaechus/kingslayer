use std::io;
use std::io::Write;

use read_line;
use room::Room;
use world::World;

/// A Command Line Interface struct;
/// controls all of the interactions between the user and all game objects
pub struct Cli {
    running: bool,
    world: World,
    // cmds: Vec<String>,
    // verbs: Vec<String>,
    // preps: Vec<String>,
    // adjs: Vec<String>,
    // nouns: Vec<String>,
}

impl Cli {
    /// Cli constructor
    pub fn new(rooms: Vec<Room>) -> Cli {
        Cli {
            running: true,
            world: World { rooms: rooms },
        }
    }
    /// starts the Cli session
    pub fn start(&self) {
        let mut name;
        loop {
            print!("Enter a character name: ");
            io::stdout().flush().expect("error flushing");
            name = read_line();
            if !name.is_empty() {
                break;
            }
        }
        let name = name;
        println!("Welcome, {}!", name);
        while self.running {
            let command = self.prompt();
            let command = self.parts(command);
            for word in command.iter() {
                print!("{}, ", word)
            }
            println!()
        }
    }
    /// prompts the user for a command
    fn prompt(&self) -> String {
        loop {
            print!("> ");
            io::stdout().flush().expect("error flushing");
            let input = read_line();
            if !input.is_empty() {
                return input;
            }
        }
    }
    /// splits a string into a vector of individual words
    fn parts(&self, s: String) -> Vec<String> {
        let out = s.split_whitespace().map(|x| x.to_owned());
        let out: Vec<String> = out.collect();
        out
    }
    //fn filter(&self, words: Vec<String>) {}
    //fn parse(&self, words: Vec<String>) {}
}
