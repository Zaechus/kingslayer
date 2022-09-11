use std::{
    collections::HashMap,
    error,
    fs::File,
    io::{self, Read, Write},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{read_line, thing::Thing, tokens::Tokens};

pub(crate) const PLAYER: &str = "PLAYER";

/// A Kingslayer game
#[derive(Default, Deserialize, Serialize)]
pub struct Game {
    player: Thing,
    things: HashMap<String, Thing>,
}

impl FromStr for Game {
    type Err = ron::error::SpannedError;

    /// Create a Game from a RON string
    /// ```
    /// # use kingslayer::Game;
    /// include_str!("world.ron").parse::<Game>();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::from_str(s)
    }
}

impl Game {
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
                    "" => "Excuse me?".to_owned(),
                    s => self.ask(s),
                }
            );
        }

        Ok(())
    }

    /// Parse a string into a game action and return the output
    /// ```
    /// # use kingslayer::Game;
    /// # let mut game: Game = include_str!("world.ron").parse().unwrap();
    /// println!("{}", game.ask("look around"));
    /// ```
    pub fn ask<S: Into<String>>(&mut self, input: S) -> String {
        self.parse(Tokens::new(input.into()))
    }

    fn parse(&mut self, tokens: Tokens) -> String {
        if let Some(verb) = tokens.verb() {
            match verb {
                "l" | "look" => self.parse_look(&tokens),
                "i" | "inventory" => self.inventory(),
                "go" | "enter" => {
                    if let Some(noun) = tokens.noun() {
                        self.go(noun)
                    } else {
                        format!("Where do you want to {}?", verb)
                    }
                }
                "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => self.go(verb),
                _ => {
                    if let Some(noun) = tokens.noun() {
                        match verb {
                            "take" | "get" => self.take(noun),
                            "drop" => self.drop(noun),
                            "open" => self.open(noun),
                            _ => {
                                format!("I do not know the word \"{}\".", verb)
                            }
                        }
                    } else {
                        format!("What do you want to {}?", verb)
                    }
                }
            }
        } else {
            "I do not understand that phrase.".to_owned()
        }
    }

    fn parse_look(&self, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            if let Some((location, _)) = self.things.iter().find(|(location, thing)| {
                (location.as_str() == self.player.location()
                    || thing.location() == self.player.location())
                    && thing.names_contains(noun)
            }) {
                self.look(location)
            } else {
                format!("You can't see any {} here.", noun)
            }
        } else {
            self.look(self.player.location())
        }
    }

    // TODO
    fn look(&self, location: &str) -> String {
        let thing = self.things.get(location).unwrap();

        if thing.is_container() {
            self.things
                .iter()
                .fold(thing.desc().to_owned(), |acc, (loc, i)| {
                    if thing.is_open() && i.location() == location && !i.desc().is_empty() {
                        if i.is_container() {
                            format!("{}\n{}", acc, self.look(loc))
                        } else if thing.is_container() && thing.is_open() {
                            format!("{}\n  a {}", acc, i.name())
                        } else {
                            format!("{}\n{}", acc, i)
                        }
                    } else {
                        acc
                    }
                })
        } else {
            self.things
                .iter()
                .fold(thing.desc().to_owned(), |acc, (loc, i)| {
                    if i.location() == location && !i.desc().is_empty() {
                        if i.is_container() {
                            format!("{}\n{}", acc, self.look(loc))
                        } else {
                            format!("{}\n{}", acc, i)
                        }
                    } else {
                        acc
                    }
                })
        }
    }

    fn inventory(&self) -> String {
        let inv = self.things.values().fold(String::new(), |acc, thing| {
            if thing.location() == PLAYER && !thing.desc().is_empty() {
                format!("{}\n  a {}", acc, thing.name())
            } else {
                acc
            }
        });

        if inv.is_empty() {
            "Your inventory is empty.".to_owned()
        } else {
            format!("You are carring:{}", inv)
        }
    }

    // TODO
    fn go(&mut self, direction: &str) -> String {
        if let Some(exit) = self.things.values().find(|thing| {
            thing.location() == self.player.location() && thing.names_contains(direction)
        }) {
            if !exit.dest().is_empty() {
                self.player.set_location(exit.dest().to_owned());
                self.look(self.player.location())
            } else if !exit.go_message().is_empty() {
                exit.go_message().to_owned()
            } else {
                "Nice try.".to_owned()
            }
        } else if matches!(
            direction,
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d"
        ) {
            "You cannot go that way.".to_owned()
        } else {
            format!("You cannot see any {} here.", direction)
        }
    }

    fn take(&mut self, noun: &str) -> String {
        if let Some(thing) = self
            .things
            .values_mut()
            .find(|thing| thing.location() == self.player.location() && thing.names_contains(noun))
        {
            thing.take().to_owned()
        } else {
            format!("There is no \"{}\" here.", noun)
        }
    }

    fn drop(&mut self, noun: &str) -> String {
        if let Some(thing) = self
            .things
            .values_mut()
            .find(|thing| thing.location() == PLAYER && thing.names_contains(noun))
        {
            thing.set_location(self.player.location().to_owned());
            "Dropped.".to_owned()
        } else {
            format!("You do not have the \"{}\".", noun)
        }
    }

    fn open(&mut self, noun: &str) -> String {
        if let Some(thing) = self.things.values_mut().find(|i| {
            i.location() == self.player.location() && i.can_open() && i.names_contains(noun)
        }) {
            thing.open();
            "Opened.".to_owned()
        } else {
            format!("There is no \"{}\" here.", noun)
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

    /// Save the Game to `kingslayer.save`.
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
