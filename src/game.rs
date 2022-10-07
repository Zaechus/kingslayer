use std::{
    collections::HashMap,
    error,
    fs::File,
    io::{self, Write},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    direction::Direction,
    item::{list_items, Container, Item},
    read_line,
    tokens::Tokens,
};

/// A Kingslayer game
#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    player: String,
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
    /// Parse a string into game actions and return the output.
    /// ```
    /// # use kingslayer::Game;
    /// # let mut game: Game = include_str!("world.ron").parse().unwrap();
    /// println!("{}", game.ask("look around"));
    /// ```
    pub fn ask<S: Into<String>>(&mut self, input: S) -> String {
        let input = input.into();
        let commands: Vec<_> = input.split("and").map(str::trim).collect();

        let first = Tokens::new(commands[0].to_string());
        let mut res = self.parse(first.clone());

        for c in commands.iter().skip(1) {
            if !c.is_empty() {
                let mut tokens = Tokens::new(c.to_string());
                if !tokens.verb().is_some() {
                    tokens = tokens.with_verb(first.verb().unwrap())
                }

                res = format!("{}\n\n{}", res, self.parse(tokens));
            }
        }

        res
    }

    fn close(&mut self, verb: &str, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            let loc = if let Some((loc, item)) = self
                .items
                .iter()
                .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(noun))
            {
                if !item.door().is_empty() {
                    item.door().to_owned()
                } else {
                    loc.to_owned()
                }
            } else {
                return cant_see_any(noun);
            };

            let item = self.item_mut(&loc);

            item.close()
        } else {
            format!("What do you want to {}?", verb)
        }
    }

    fn contents(&self, location: &str, item: &Item) -> String {
        if item.is_open() {
            let contents = self.items.values().fold(String::new(), |acc, i| {
                if i.is_in(location) && !i.name().is_empty() {
                    format!("{}\n  a {}", acc, i.name())
                } else {
                    acc
                }
            });

            if contents.is_empty() {
                String::new()
            } else {
                format!("The {} contains:{}", item.name(), contents)
            }
        } else {
            String::new()
        }
    }

    fn desc_contents(&self, location: &str, item: &Item) -> String {
        let contents = self.contents(location, item);

        if contents.is_empty() {
            item.desc().to_owned()
        } else if item.desc().is_empty() {
            contents
        } else {
            format!("{}\n{}", item.desc(), contents)
        }
    }

    fn drop(&mut self, noun: &str) -> String {
        let player_location = self.player_location().to_owned();

        if let Some(item) = self
            .items
            .values_mut()
            .find(|i| i.is_in(&self.player) && i.names_contains(noun))
        {
            item.set_location(player_location);
            "Dropped.".to_owned()
        } else {
            format!("You do not have the {}.", noun)
        }
    }

    fn examine(&self, location: &str, item: &Item) -> String {
        match item.container() {
            Container::Open | Container::True => {
                let contents = self.contents(location, item);

                if contents.is_empty() {
                    format!("The {} is empty.", item.name())
                } else {
                    contents
                }
            }
            Container::Closed => format!("The {} is closed.", item.name()),
            _ => {
                if item.location().is_empty() {
                    self.look()
                } else {
                    format!("There is nothing remarkable about the {}.", item.name())
                }
            }
        }
    }

    fn go(&mut self, direction: &str) -> String {
        if let Some((_, exit)) = self
            .items
            .iter()
            .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(direction))
        {
            if !exit.dest().is_empty() {
                let exit_dest = exit.dest().to_owned();

                if let Some(door) = self.items.get(exit.door()) {
                    if door.is_open() {
                        self.item_mut(&self.player.clone()).set_location(exit_dest);
                        self.look()
                    } else {
                        format!("The {} is closed.", door.name())
                    }
                } else {
                    self.item_mut(&self.player.clone()).set_location(exit_dest);
                    self.look()
                }
            } else if !exit.go_message().is_empty() {
                exit.go_message().to_owned()
            } else {
                "Nice try.".to_owned()
            }
        } else if direction.is_direction() || direction == "enter" {
            "You cannot go that way.".to_owned()
        } else {
            cant_see_any(direction)
        }
    }

    fn in_room(&self, loc: &str, i: &Item) -> bool {
        loc == self.player
            || i.is_in(self.player_location())
            || if let Some(parent) = self.items.get(i.location()) {
                parent.is_open() && parent.is_in(self.player_location())
            } else {
                false
            }
    }

    fn inventory(&self) -> String {
        let inv = self.items.values().fold(String::new(), |acc, item| {
            if item.is_in(&self.player) {
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

    fn is_visible(&self, loc: &str, i: &Item) -> bool {
        i.is_in(&self.player) || self.in_room(loc, i)
    }

    fn item(&self, key: &str) -> &Item {
        self.items.get(key).unwrap()
    }

    fn item_mut(&mut self, key: &str) -> &mut Item {
        self.items.get_mut(key).unwrap()
    }

    /// Load a Game from a savefile
    /// ```
    /// # use kingslayer::Game;
    /// # let game = Game::default();
    /// # game.save("kingslayer.save");
    /// Game::load("kingslayer.save");
    /// ```
    pub fn load(filename: &str) -> Result<Self, Box<dyn error::Error>> {
        let mut bytes = Vec::new();
        zstd::stream::copy_decode(File::open(filename)?, &mut bytes)?;

        Ok(bincode::deserialize(&bytes)?)
    }

    fn look(&self) -> String {
        let room = self.item(self.player_location());

        format!(
            "{}{}{}",
            if !room.name().is_empty() {
                format!("{}\n", room.name())
            } else {
                String::new()
            },
            room.desc(),
            {
                let contents = self.items.iter().fold(String::new(), |acc, (loc, i)| {
                    if i.is_in(self.player_location()) {
                        let desc = if i.is_container() {
                            self.desc_contents(loc, i)
                        } else {
                            i.desc().to_owned()
                        };

                        if desc.is_empty() {
                            acc
                        } else {
                            format!("{}\n{}", acc, desc)
                        }
                    } else {
                        acc
                    }
                });

                contents
            }
        )
    }

    fn open(&mut self, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            let loc = if let Some((loc, item)) = self
                .items
                .iter()
                .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(noun))
            {
                if !item.door().is_empty() {
                    item.door().to_owned()
                } else {
                    loc.to_owned()
                }
            } else {
                return cant_see_any(noun);
            };

            let reveals = list_items(
                &self
                    .items
                    .values()
                    .filter(|i| i.is_in(&loc))
                    .collect::<Vec<_>>(),
            );

            self.item_mut(&loc).open(reveals)
        } else {
            "What do you want to open?".to_owned()
        }
    }

    fn parse(&mut self, tokens: Tokens) -> String {
        if let Some(verb) = tokens.verb() {
            match verb {
                _ if verb.is_direction() => self.go(verb),
                "again" | "g" => todo!("repeat last command"),
                "attack" | "cut" | "hit" | "hurt" | "kill" => todo!("attack something"),
                "break" | "destroy" => todo!("try to break something"),
                "burn" => todo!("try to burn something"),
                "climb" | "scale" => todo!("climb"),
                "close" | "shut" => self.close(verb, &tokens),
                "drop" | "place" | "put" | "throw" => self.parse_drop(verb, &tokens),
                "eat" | "drink" => todo!("try to eat something"),
                "inventory" | "i" => self.inventory(),
                "go" | "enter" | "walk" => self.parse_go(verb, &tokens),
                "look" | "l" | "examine" | "inspect" | "read" | "what" => {
                    self.parse_look(verb, &tokens)
                }
                "move" | "pull" | "push" => todo!("try to move item"),
                "open" => self.open(&tokens),
                "take" | "hold" | "get" | "pick" | "remove" => self.parse_take(verb, &tokens),
                "wait" | "z" => "Time passes...".to_owned(),
                "where" | "see" => self.where_is(&tokens),
                _ => {
                    format!("I do not know the word \"{}\".", verb)
                }
            }
        } else {
            "I do not understand that phrase.".to_owned()
        }
    }

    fn parse_drop(&mut self, verb: &str, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            if let Some(prep) = tokens.prep() {
                if let Some(obj) = tokens.obj() {
                    self.put(noun, obj)
                } else {
                    format!("What do you want to {} the {} {}?", verb, noun, prep)
                }
            } else {
                match verb {
                    "drop" => self.drop(noun),
                    _ => format!("Where do you want to {} the {}?", verb, noun),
                }
            }
        } else if let Some(prep) = tokens.prep() {
            if let Some(obj) = tokens.obj() {
                format!("What do you want to {} {} the {}?", verb, prep, obj)
            } else {
                format!("What do you want to {}?", verb)
            }
        } else {
            format!("What do you want to {}?", verb)
        }
    }

    fn parse_go(&mut self, verb: &str, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            self.go(noun)
        } else if verb == "enter" {
            self.go(verb)
        } else {
            format!("Where do you want to {}?", verb)
        }
    }

    fn parse_look(&self, verb: &str, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            if let Some((_, item)) = self.items.iter().find(|(loc, i)| {
                self.is_visible(loc, i) && i.names_contains(noun) && !i.what().is_empty()
            }) {
                item.what().to_owned()
            } else if let Some((loc, item)) = self
                .items
                .iter()
                .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(noun))
            {
                self.examine(loc, item)
            } else {
                cant_see_any(noun)
            }
        } else {
            match verb {
                "l" | "look" => self.look(),
                _ => format!("What do you want to {}?", verb),
            }
        }
    }

    fn parse_take(&mut self, verb: &str, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            if noun == "all" || noun == "everything" {
                self.take_all()
            } else if let Some((loc, _)) = self
                .items
                .iter()
                .find(|(loc, i)| self.in_room(loc, i) && i.names_contains(noun))
            {
                self.items
                    .get_mut(&loc.to_owned())
                    .unwrap()
                    .take(&self.player)
                    .to_owned()
            } else if self
                .items
                .values()
                .any(|i| i.location() == self.player && i.names_contains(noun))
            {
                "You already have that!".to_owned()
            } else {
                cant_see_any(noun)
            }
        } else {
            format!("What do you want to {}?", verb)
        }
    }

    /// Start the Game in a command line setting where `print` macros are expected to work
    pub fn play(&mut self) -> Result<(), Box<dyn error::Error>> {
        println!("{}", self.ask("l"));

        loop {
            println!(
                "{}",
                match prompt("\n> ")?.trim() {
                    "quit" | "q" => break,
                    "restore" => self.restore("kingslayer.save")?,
                    "save" => self.save("kingslayer.save")?,
                    "" => "Excuse me?".to_owned(),
                    s => self.ask(s),
                }
            );
        }

        Ok(())
    }

    fn player_location(&self) -> &str {
        self.item(&self.player).location()
    }

    fn put(&mut self, noun: &str, obj: &str) -> String {
        let container = if let Some((loc, _)) = self
            .items
            .iter()
            .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(obj))
        {
            loc.to_owned()
        } else {
            return cant_see_any(noun);
        };

        let item = if let Some((loc, _)) = self
            .items
            .iter()
            .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(noun))
        {
            loc.to_owned()
        } else {
            return cant_see_any(noun);
        };

        match self.item(&container).container() {
            Container::Open | Container::True => {
                if container == item {
                    "Impossible.".to_owned()
                } else if self.item(&item).location() == container {
                    format!(
                        "The {} is already in the {}.",
                        self.item(&item).name(),
                        self.item(&container).name()
                    )
                } else {
                    self.item_mut(&item).set_location(container);
                    "Done.".to_owned()
                }
            }
            Container::Closed => format!("The {} isn't open.", self.item(&container).name()),
            Container::False => "You can't do that.".to_owned(),
        }
    }

    /// Restore a Game from a file.
    /// ```
    /// # use kingslayer::Game;
    /// # let mut game = Game::default();
    /// game.restore("kingslayer.save");
    /// ```
    pub fn restore(&mut self, filename: &str) -> Result<String, Box<dyn error::Error>> {
        let game = Game::load(filename)?;
        self.player = game.player;
        self.items = game.items;

        Ok("OK".to_owned())
    }

    /// Save the Game to a file.
    /// ```
    /// # use kingslayer::Game;
    /// # let game = Game::default();
    /// game.save("kingslayer.save");
    /// ```
    pub fn save(&self, filename: &str) -> Result<String, Box<dyn error::Error>> {
        match File::create(filename) {
            Ok(file) => {
                zstd::stream::copy_encode(&*bincode::serialize(&self)?, file, 3).unwrap();
                Ok("Saved.".to_owned())
            }
            Err(e) => Ok(e.to_string()),
        }
    }

    fn take_all(&mut self) -> String {
        let items = self.items.iter().fold(Vec::new(), |mut acc, (loc, i)| {
            if i.is_in(self.player_location()) && !i.name().is_empty() && !i.desc().is_empty() {
                acc.push(loc.to_owned())
            }
            acc
        });

        let player = self.player.clone();
        let message = items
            .iter()
            .fold(String::new(), |acc, loc| {
                let item = self.item_mut(loc);
                format!(
                    "{}\n{}: {}",
                    acc,
                    item.name().to_owned(),
                    item.take(&player)
                )
            })
            .trim()
            .to_owned();

        if message.is_empty() {
            "You can't see anything you can take.".to_owned()
        } else {
            message
        }
    }

    fn where_is(&self, tokens: &Tokens) -> String {
        if let Some(noun) = tokens.noun() {
            if noun == "i" {
                self.item(self.player_location()).desc().to_owned()
            } else if self
                .items
                .iter()
                .any(|(loc, i)| self.is_visible(loc, i) && i.names_contains(noun))
            {
                "It's here.".to_owned()
            } else {
                cant_see_any(noun)
            }
        } else {
            "Excuse me?".to_owned()
        }
    }
}

fn cant_see_any(noun: &str) -> String {
    format!("You can't see any {} here.", noun)
}
