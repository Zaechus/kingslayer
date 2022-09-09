use std::{
    collections::HashMap,
    error,
    fs::File,
    io::{self, Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::{player::Player, read_line, room::Room, tokens::Tokens};

/// A Kingslayer game
#[derive(Deserialize, Serialize)]
pub struct Game {
    player: Player,
    rooms: HashMap<String, Room>,
}

impl Game {
    pub fn new(player: Player, rooms: HashMap<String, Room>) -> Self {
        Self { player, rooms }
    }

    /// Start the Game in a command line setting where `print` macros are expected to work
    pub fn play(&mut self) -> Result<(), Box<dyn error::Error>> {
        println!("{}", self.ask("l"));

        loop {
            print!("\n> ");
            io::stdout().flush()?;

            println!(
                "{}",
                match read_line()?.trim() {
                    "quit" => break,
                    "save" => self.save()?,
                    s => self.ask(s),
                }
            );
        }

        Ok(())
    }

    pub fn ask<S: Into<String>>(&mut self, input: S) -> String {
        let command = Tokens::new(input.into());

        if let Some(verb) = command.verb() {
            match verb {
                "i" | "inventory" => self.player.inventory().to_string(),
                "l" | "look" => self.rooms.get(self.player.location()).unwrap().to_string(),
                _ => format!("I do not know the word \"{}\".", verb),
            }
        } else {
            "I do not understand that phrase.".to_owned()
        }
    }

    /// Restore a Game from a savefile
    pub fn restore(filename: &str) -> Result<Self, Box<dyn error::Error>> {
        let mut file = File::open(filename)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(bincode::deserialize(&bytes)?)
    }

    /// Save the Game to kingslayer.save
    pub fn save(&self) -> Result<String, Box<dyn error::Error>> {
        match File::create("kingslayer.save") {
            Ok(mut file) => {
                file.write_all(&bincode::serialize(&self)?)?;
                Ok("Saved.".to_owned())
            }
            Err(e) => Ok(e.to_string()),
        }
    }
}
