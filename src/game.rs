use std::{
    cell::Cell,
    collections::HashMap,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::{
    entity::room::Room,
    lexer::lex,
    parse::{parse_drop, parse_take},
    player::Player,
    read_line,
};

/// An interface to a Kingslayer game
#[derive(Deserialize, Serialize)]
pub struct Game {
    #[serde(default)]
    running: Cell<bool>,
    #[serde(default)]
    num_moves: Cell<u32>,
    #[serde(default)]
    player: Player,
    curr_room: String,
    rooms: HashMap<String, Room>,
}

impl Default for Game {
    fn default() -> Self {
        ron::de::from_str(include_str!("worlds/world.ron")).expect("RON error")
    }
}

impl Game {
    /// Setup a game from RON
    pub fn from_ron_str(ron_str: &str) -> Self {
        ron::de::from_str(ron_str).expect("RON error")
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

    fn move_room(&mut self, direction: &str) -> String {
        if let Some(path) = self
            .rooms
            .get(&self.curr_room)
            .unwrap()
            .find_path(direction)
        {
            self.curr_room = path.target().to_owned();
            self.rooms.get(&self.curr_room).unwrap().to_string()
        } else {
            "You cannot go that way.".to_owned()
        }
    }

    pub fn ask(&mut self, input: String) -> String {
        let command = lex(input);

        if let Some(verb) = command.verb() {
            match verb {
                "north" | "south" | "east" | "west" | "northeast" | "northwest" | "southeast"
                | "southwest" | "up" | "down" => self.move_room(verb),
                "enter" | "go" | "move" | "exit" => {
                    if let Some(obj) = command.obj() {
                        self.move_room(obj)
                    } else {
                        format!("Where do you want to {}?", verb)
                    }
                }
                "drop" => parse_drop(
                    verb,
                    &command,
                    &mut self.player,
                    self.rooms.get_mut(&self.curr_room).unwrap(),
                ),
                "i" => self.player.inventory().to_string(),
                "l" | "look" => self.rooms.get(&self.curr_room).unwrap().to_string(),
                "take" => parse_take(
                    verb,
                    &command,
                    &mut self.player,
                    self.rooms.get_mut(&self.curr_room).unwrap(),
                ),
                "quit" => {
                    self.running.set(false);
                    "\nFarewell.\n".to_owned()
                }
                "save" => self.save(),
                _ => format!("I do not know the word \"{}\".", verb),
            }
        } else {
            "I do not understand that phrase.".to_owned()
        }
    }

    /// Start a game from the command line
    pub fn play(&mut self) -> io::Result<()> {
        self.running.set(true);

        println!("{}", self.ask("l".into()));

        while self.running.get() {
            print!("\n> ");
            io::stdout().flush()?;

            println!("{}", self.ask(read_line()?));
        }

        Ok(())
    }
}
