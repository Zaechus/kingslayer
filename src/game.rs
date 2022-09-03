use std::{
    cell::Cell,
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::{player::Player, read_line, room::Room};

/// An interface to a Kingslayer game
#[derive(Deserialize, Serialize)]
pub struct Game {
    #[serde(default)]
    running: Cell<bool>,
    #[serde(default)]
    num_moves: Cell<u32>,
    #[serde(default)]
    player: Player,
    rooms: HashMap<String, Room>,
    curr_room: String,
}

impl Game {
    /// Setup a game from RON
    pub fn from_ron_str(ron_str: &str) -> Self {
        ron::de::from_str(ron_str).unwrap()
    }

    /// Load a game from a savefile
    pub fn load(filename: &str) -> Self {
        let mut reader = BufReader::new(File::open(filename).unwrap());
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).unwrap();

        bincode::deserialize(&bytes).unwrap()
    }

    /// Save the game to the current directory as kingslayer.save
    fn save(&self) -> io::Result<&str> {
        match File::create("kingslayer.save") {
            Ok(mut file) => {
                file.write_all(&bincode::serialize(&self).unwrap())?;
                Ok("Saved.")
            }
            Err(e) => Err(e),
        }
    }

    fn ask(&self, input: &str) -> String {
        match input.trim() {
            "i" => self.player.inventory().to_string(),
            "l" => self.rooms.get(&self.curr_room).unwrap().to_string(),
            "quit" => {
                self.running.set(false);
                "\nFarewell".to_owned()
            }
            "save" => self.save().unwrap().to_owned(),
            _ => "Excuse me?".to_owned(),
        }
    }

    /// Start a normal game from the command line
    pub fn play(&self) {
        self.running.set(true);

        while self.running.get() {
            print!("\n> ");
            io::stdout().flush().unwrap();

            println!("{}", self.ask(&read_line()));
        }
    }
}
