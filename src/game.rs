use std::{
    cell::Cell,
    collections::HashMap,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::{lexer::lex, player::Player, read_line, room::Room};

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
    pub fn load(filename: &str) -> io::Result<Self> {
        let mut reader = BufReader::new(File::open(filename)?);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        Ok(bincode::deserialize(&bytes).unwrap())
    }

    /// Save the game to the current directory as kingslayer.save
    fn save(&self) -> String {
        match File::create("kingslayer.save") {
            Ok(file) => {
                let mut writer = BufWriter::new(file);
                writer
                    .write_all(&bincode::serialize(&self).unwrap())
                    .unwrap();
                "Saved.".to_owned()
            }
            Err(e) => format!("error: {}", e),
        }
    }

    fn ask(&mut self, input: String) -> String {
        let command = lex(input);

        match command.verb().unwrap_or_default() {
            "drop" => self
                .rooms
                .get_mut(&self.curr_room)
                .unwrap()
                .take(self.player.drop(command.obj().unwrap())),
            "i" => self.player.inventory().to_string(),
            "l" => self.rooms.get(&self.curr_room).unwrap().to_string(),
            "take" => self.player.take(
                self.rooms
                    .get_mut(&self.curr_room)
                    .unwrap()
                    .give(command.obj().unwrap()),
            ),
            "quit" => {
                self.running.set(false);
                "\nFarewell.\n".to_owned()
            }
            "save" => self.save(),
            _ => "Excuse me?".to_owned(),
        }
    }

    /// Start a normal game from the command line
    pub fn play(&mut self) {
        self.running.set(true);

        while self.running.get() {
            print!("\n> ");
            io::stdout().flush().unwrap();

            println!("{}", self.ask(read_line()));
        }
    }
}
