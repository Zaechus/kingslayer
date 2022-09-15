use std::{
    collections::HashMap,
    error,
    fs::File,
    io::{self, Read, Write},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    item::{list_items, Container, Item},
    read_line,
    tokens::Tokens,
};

const BINCODE_SHIFT: u8 = 59;
pub(crate) const PLAYER: &str = "PLAYER";

/// A Kingslayer game
#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    player: Item,
    items: HashMap<String, Item>,
}

impl Default for Game {
    fn default() -> Self {
        ron::from_str(include_str!("world.ron")).unwrap()
    }
}

impl FromStr for Game {
    type Err = ron::error::SpannedError;

    /// Create a Game from a RON string
    /// ```
    /// # use kingslayer::Game;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let game: Game = include_str!("world.ron").parse()?;
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ron::from_str(s)
    }
}

fn prompt(p: &str) -> io::Result<String> {
    print!("{}", p);
    io::stdout().flush()?;
    read_line()
}

impl Game {
    /// Start the Game in a command line setting where `print` macros are expected to work
    pub fn play(&mut self) -> Result<(), Box<dyn error::Error>> {
        println!("{}", self.ask("l"));

        loop {
            println!(
                "{}",
                match prompt("\n> ")?.trim() {
                    "quit" => break,
                    "save" => self.save("kingslayer.save")?,
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
                            "examine" | "inspect" | "read" | "what" => self.examine(noun),
                            "take" | "get" => self.parse_take(noun),
                            "drop" => self.drop(noun),
                            "open" => self.open(noun),
                            "close" => self.close(noun),
                            "put" | "place" => todo!("put"),
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
            if let Some((loc, item)) = self.items.iter().find(|(loc, i)| {
                self.is_visible(loc, i) && i.names_contains(noun) && !i.desc().is_empty()
            }) {
                self.look(loc, item)
            } else if let Some((loc, item)) = self
                .items
                .iter()
                .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(noun))
            {
                self.look(loc, item)
            } else {
                cant_see_any(noun)
            }
        } else {
            self.look(
                self.player.location(),
                self.items.get(self.player.location()).unwrap(),
            )
        }
    }

    // TODO: its so ugly but it works...
    fn look(&self, location: &str, item: &Item) -> String {
        if item.is_container() {
            let contents = self.items.iter().fold(String::new(), |acc, (loc, i)| {
                if item.is_open() && i.is_in(location) && !i.desc().is_empty() {
                    if i.is_container() {
                        format!("{}\n{}", acc, self.look(loc, i))
                    } else {
                        format!("{}\n  a {}", acc, i.name())
                    }
                } else {
                    acc
                }
            });

            if contents.is_empty() {
                item.desc().to_owned()
            } else {
                format!("{} It contains:{}", item.desc(), contents)
            }
        } else {
            self.items.iter().fold(
                if item.location().is_empty() && !item.name().is_empty() {
                    format!(
                        "{}\n{}",
                        item.name(),
                        if item.desc().is_empty() {
                            item.what()
                        } else {
                            item.desc()
                        }
                    )
                } else {
                    item.desc().to_owned()
                },
                |acc, (loc, i)| {
                    if i.is_in(location) && !i.desc().is_empty() {
                        if i.is_container() {
                            format!("{}\n{}", acc, self.look(loc, i))
                        } else {
                            format!("{}\n{}", acc, i.desc())
                        }
                    } else {
                        acc
                    }
                },
            )
        }
    }

    fn inventory(&self) -> String {
        let inv = self.items.values().fold(String::new(), |acc, item| {
            if item.is_in(PLAYER) {
                format!("{}\n  a {}", acc, item.name())
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
        if let Some(exit) = self
            .items
            .values()
            .find(|item| item.is_in(self.player.location()) && item.names_contains(direction))
        {
            if !exit.dest().is_empty() {
                if let Some(door) = self.items.get(exit.door()) {
                    if door.is_open() {
                        self.player.set_location(exit.dest().to_owned());
                        self.look(
                            self.player.location(),
                            self.items.get(self.player.location()).unwrap(),
                        )
                    } else {
                        format!("The {} is closed.", door.name())
                    }
                } else {
                    self.player.set_location(exit.dest().to_owned());
                    self.look(
                        self.player.location(),
                        self.items.get(self.player.location()).unwrap(),
                    )
                }
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

    fn is_visible(&self, loc: &str, i: &Item) -> bool {
        loc == self.player.location()
            || i.is_in(PLAYER)
            || i.is_in(self.player.location())
            || if let Some(parent) = self.items.get(i.location()) {
                parent.is_open() && parent.is_in(self.player.location())
            } else {
                false
            }
    }

    fn in_room(&self, i: &Item) -> bool {
        i.is_in(self.player.location())
            || if let Some(parent) = self.items.get(i.location()) {
                parent.is_open() && parent.is_in(self.player.location())
            } else {
                false
            }
    }

    // TODO
    fn examine(&self, noun: &str) -> String {
        if let Some((_, item)) = self.items.iter().find(|(loc, i)| {
            self.is_visible(loc, i) && i.names_contains(noun) && !i.what().is_empty()
        }) {
            item.what().to_owned()
        } else if let Some((loc, item)) = self
            .items
            .iter()
            .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(noun))
        {
            match item.container() {
                Container::Open | Container::True => self.look(loc, item),
                Container::Closed => format!("The {} is closed.", item.name()),
                _ => format!("There is nothing remarkable about the {}.", item.name()),
            }
        } else {
            cant_see_any(noun)
        }
    }

    fn parse_take(&mut self, noun: &str) -> String {
        if noun == "all" || noun == "everything" {
            self.take_all()
        } else if let Some((loc, _)) = self
            .items
            .iter()
            .find(|(_, i)| self.in_room(i) && i.names_contains(noun))
        {
            self.items
                .get_mut(&loc.to_owned())
                .unwrap()
                .take()
                .to_owned()
        } else if self
            .items
            .values()
            .any(|i| i.location() == PLAYER && i.names_contains(noun))
        {
            "You already have that!".to_owned()
        } else {
            cant_see_any(noun)
        }
    }

    fn take_all(&mut self) -> String {
        let message = self
            .items
            .values_mut()
            .fold(String::new(), |acc, i| {
                if i.is_in(self.player.location()) && !i.desc().is_empty() {
                    format!("{}\n{}: {}", acc, i.name().to_owned(), i.take())
                } else {
                    acc
                }
            })
            .trim()
            .to_owned();

        if message.is_empty() {
            "You can't see anything you can take.".to_owned()
        } else {
            message
        }
    }

    fn drop(&mut self, noun: &str) -> String {
        if let Some(item) = self
            .items
            .values_mut()
            .find(|item| item.is_in(PLAYER) && item.names_contains(noun))
        {
            item.set_location(self.player.location().to_owned());
            "Dropped.".to_owned()
        } else {
            format!("You do not have the \"{}\".", noun)
        }
    }

    // TODO
    fn open(&mut self, noun: &str) -> String {
        let (loc, name) = if let Some((loc, item)) = self
            .items
            .iter_mut()
            .find(|(_, i)| i.is_in(self.player.location()) && i.names_contains(noun))
        {
            if item.is_open() {
                return format!("The {} is already open.", item.name());
            } else if item.can_open() {
                (loc.to_owned(), item.name().to_owned())
            } else if !item.door().is_empty() {
                (item.door().to_owned(), item.name().to_owned())
            } else if !item.open_message().is_empty() {
                return item.open_message().to_owned();
            } else {
                return format!("The {} cannot be opened.", item.name());
            }
        } else {
            return cant_see_any(noun);
        };

        self.items.get_mut(&loc).unwrap().open();

        let reveals = list_items(self.items.values().filter(|i| i.is_in(&loc)).collect());

        if reveals.is_empty() {
            "Opened.".to_owned()
        } else {
            format!("Opening the {} reveals {}.", name, reveals)
        }
    }

    fn close(&mut self, noun: &str) -> String {
        if let Some(item) = self
            .items
            .values_mut()
            .find(|i| i.is_in(self.player.location()) && i.can_open() && i.names_contains(noun))
        {
            item.close()
        } else {
            format!("There is no \"{}\" here.", noun)
        }
    }

    /// Restore a Game from a savefile
    /// ```
    /// # use kingslayer::Game;
    /// # let game = Game::default();
    /// # game.save("kingslayer.save");
    /// Game::restore("kingslayer.save");
    /// ```
    pub fn restore(filename: &str) -> Result<Self, Box<dyn error::Error>> {
        let mut file = File::open(filename)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(bincode::deserialize(
            &bytes
                .into_iter()
                .map(|x| x.wrapping_sub(BINCODE_SHIFT))
                .collect::<Vec<u8>>(),
        )?)
    }

    /// Save the Game to a file.
    /// ```
    /// # use kingslayer::Game;
    /// # let game = Game::default();
    /// game.save("kingslayer.save");
    /// ```
    pub fn save(&self, filename: &str) -> Result<String, Box<dyn error::Error>> {
        match File::create(filename) {
            Ok(mut file) => {
                file.write_all(
                    &bincode::serialize(&self)?
                        .into_iter()
                        .map(|x| x.wrapping_add(BINCODE_SHIFT))
                        .collect::<Vec<u8>>(),
                )?;
                Ok("Saved.".to_owned())
            }
            Err(e) => Ok(e.to_string()),
        }
    }
}

fn cant_see_any(noun: &str) -> String {
    format!("You can't see any {} here.", noun)
}
