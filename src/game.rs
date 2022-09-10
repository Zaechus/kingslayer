use std::{
    collections::HashMap,
    error,
    fs::File,
    io::{self, Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::{read_line, thing::Thing, tokens::Tokens};

/// A Kingslayer game
#[derive(Default, Deserialize, Serialize)]
pub struct Game {
    player: Thing,
    things: HashMap<String, Thing>,
}

impl Game {
    pub fn new<S: Into<String>>(start: S) -> Self {
        Self {
            player: Thing::default().with_location(start),
            things: HashMap::new(),
        }
    }

    pub fn with<S: Into<String>>(mut self, key: S, thing: Thing) -> Self {
        self.things.insert(key.into(), thing);
        self
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

    /// Parse a string into a game action and return the output
    pub fn ask<S: Into<String>>(&mut self, input: S) -> String {
        self.parse(Tokens::new(input.into()))
    }

    fn parse(&mut self, tokens: Tokens) -> String {
        if let Some(verb) = tokens.verb() {
            match verb {
                "l" | "look" => self.look(),
                "i" | "inventory" => self.inventory(),
                "go" => {
                    if let Some(noun) = tokens.noun() {
                        self.go(noun)
                    } else {
                        "Where do you want to go?".to_owned()
                    }
                }
                "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => self.go(verb),
                "take" | "get" => self.parse_take(verb, &tokens),
                "drop" => self.parse_drop(verb, &tokens),
                _ => {
                    format!("I do not know the word \"{}\".", verb)
                }
            }
        } else {
            "I do not understand that phrase.".to_owned()
        }
    }

    fn look(&self) -> String {
        self.things.values().fold(
            self.things
                .get(self.player.location())
                .unwrap()
                .desc()
                .to_owned(),
            |acc, thing| {
                if thing.location() == self.player.location() && !thing.desc().is_empty() {
                    format!("{}\n{}", acc, thing)
                } else {
                    acc
                }
            },
        )
    }

    fn inventory(&self) -> String {
        self.things
            .values()
            .fold(String::from("You are carrying:"), |acc, thing| {
                if thing.location() == "player" && !thing.desc().is_empty() {
                    format!("{}\n  a {}", acc, thing.name())
                } else {
                    acc
                }
            })
    }

    // TODO
    fn go(&mut self, direction: &str) -> String {
        if let Some(exit) = self.things.values().find(|thing| {
            thing.location() == self.player.location() && thing.names_contains(direction)
        }) {
            if let Some(dest) = exit.dest() {
                self.player.set_location(dest.to_owned());
                self.look()
            } else {
                "Nice try.".to_owned()
            }
        } else {
            "You cannot go that way.".to_owned()
        }
    }

    fn parse_take(&mut self, verb: &str, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            if let Some(thing) = self
                .things
                .values_mut()
                .find(|thing| thing.names_contains(noun))
            {
                thing.take().to_owned()
            } else {
                format!("There is no \"{}\" here.", noun)
            }
        } else {
            format!("What do you want to {}?", verb)
        }
    }

    fn parse_drop(&mut self, verb: &str, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            if let Some(thing) = self
                .things
                .values_mut()
                .find(|thing| thing.location() == "player" && thing.names_contains(noun))
            {
                thing.set_location(self.player.location().to_owned());
                "Dropped.".to_owned()
            } else {
                format!("You do not have the \"{}\".", noun)
            }
        } else {
            format!("What do you want to {}?", verb)
        }
    }

    /// Restore a Game from a savefile
    /// ```
    /// # use kingslayer::Game;
    /// # let game = Game::default();
    /// # game.save();
    /// Game::restore("kingslayer.save");
    /// ```
    pub fn restore(filename: &str) -> Result<Self, Box<dyn error::Error>> {
        let mut file = File::open(filename)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(bincode::deserialize(&bytes)?)
    }

    /// Save the Game to kingslayer.save
    /// ```
    /// # use kingslayer::Game;
    /// # let game = Game::default();
    /// game.save();
    /// ```
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
