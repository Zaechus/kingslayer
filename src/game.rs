use std::{
    collections::HashMap,
    error,
    fs::File,
    io::{self, Write},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    action::Action,
    container::Container,
    direction::Direction,
    item::{list_items, Item},
    read_line,
    tokens::Tokens,
};

/// A Kingslayer game
#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    player: String,
    items: HashMap<String, Item>,
    #[serde(default)]
    last_command: Tokens,
    #[serde(default)]
    last_it: String,
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

impl Game {
    /// Parse a string into game actions and return the output.
    /// ```
    /// # use kingslayer::Game;
    /// # let mut game: Game = include_str!("world.ron").parse().unwrap();
    /// println!("{}", game.ask("look around"));
    /// ```
    pub fn ask<S: Into<String>>(&mut self, input: S) -> String {
        let commands: Vec<_> = input
            .into()
            .replace(',', " and ")
            .replace('.', " and ")
            .replace(';', " and ")
            .split_whitespace()
            .map(str::to_lowercase)
            .fold(vec![Vec::new()], |mut acc, w| {
                if w == "and" {
                    acc.push(Vec::new())
                } else {
                    acc.last_mut().unwrap().push(w)
                }
                acc
            })
            .into_iter()
            .filter(|v| !v.is_empty())
            .collect();

        let mut res = if let Some(first) = commands.first() {
            let mut tokens = Tokens::new(first);

            if let Action::Clarify(_) = self.last_command.action() {
                if let Action::Unknown(_) = tokens.action() {
                    let phrase = if tokens.verb() == "it" {
                        self.last_it.clone()
                    } else {
                        first.join(" ")
                    };

                    if self.last_command.noun().is_empty() {
                        tokens = Tokens::with(
                            self.last_command.verb().to_owned(),
                            phrase,
                            self.last_command.prep().to_owned(),
                            self.last_command.obj().to_owned(),
                        );
                    } else {
                        tokens = Tokens::with(
                            self.last_command.verb().to_owned(),
                            self.last_command.noun().to_owned(),
                            self.last_command.prep().to_owned(),
                            phrase,
                        );
                    }
                }

                self.update_last(tokens.clone());

                self.parse(tokens.action())
            } else {
                let tokens = self.replace_it(tokens);

                match tokens.action() {
                    Action::Again | Action::Unknown(_) => (),
                    Action::Clarify(_) => self.last_command = tokens.clone(),
                    _ => {
                        self.update_last(tokens.clone());
                    }
                }

                self.parse(tokens.action())
            }
        } else {
            "Excuse me?".to_owned()
        };

        for words in commands.iter().skip(1) {
            if let Action::Clarify(_) = self.last_command.action() {
                break;
            } else {
                let mut tokens = self.replace_it(Tokens::new(words));

                if let Action::Unknown(_) = tokens.action() {
                    tokens = Tokens::with(
                        self.last_command.verb().to_owned(),
                        words.join(" "),
                        self.last_command.prep().to_owned(),
                        self.last_command.obj().to_owned(),
                    );
                }

                self.update_last(tokens.clone());

                res = format!("{}\n\n{}", res, self.parse(tokens.action()));
            }
        }

        res
    }

    fn close(&mut self, noun: &str) -> String {
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

    fn examine(&self, noun: &str) -> String {
        if let Some((_, item)) = self.items.iter().find(|(loc, i)| {
            self.is_visible(loc, i) && i.names_contains(noun) && !i.details().is_empty()
        }) {
            item.details().to_owned()
        } else if let Some((loc, item)) = self
            .items
            .iter()
            .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(noun))
        {
            match item.container() {
                Container::Open | Container::True => {
                    let contents = self.contents(loc, item);

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
        } else {
            cant_see_any(noun)
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

    fn open(&mut self, noun: &str) -> String {
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

        let contents = self
            .items
            .values()
            .filter(|i| i.is_in(&loc))
            .collect::<Vec<_>>();
        if self.item(&loc).is_closed() && contents.len() == 1 {
            self.last_it = contents[0].name().to_owned();
        }
        let reveals = list_items(&contents);

        self.item_mut(&loc).open(reveals)
    }

    fn parse(&mut self, action: &Action) -> String {
        match action {
            Action::Again => self.parse(&self.last_command.action().clone()),
            Action::Attack(_, _) => "You can't do that yet.".to_owned(),
            Action::Break(_) => "You can't do that yet.".to_owned(),
            Action::Burn(_, _) => "You can't do that yet.".to_owned(),
            Action::Clarify(message) => message.to_owned(),
            Action::Climb => "You can't do that yet.".to_owned(),
            Action::Close(noun) => self.close(noun),
            Action::Drop(noun) => self.drop(noun),
            Action::Put(noun, obj) => self.put(noun, obj),
            Action::Eat(_) => "You cannot eat that.".to_owned(),
            Action::Examine(noun) => self.examine(noun),
            Action::Hello => "Hello!".to_owned(),
            Action::Help => "That would be nice, wouldn't it?".to_owned(),
            Action::Inventory => self.inventory(),
            Action::Look => self.look(),
            Action::Move(noun) => self.parse_move(noun),
            Action::NoVerb => "Excuse me?".to_owned(),
            Action::Open(noun) => self.open(noun),
            Action::Sleep => "Time passes...".to_owned(),
            Action::Take(noun) => self.parse_take(noun),
            Action::Unknown(verb) => format!("I do not know the verb \"{}\".", verb),
            Action::Walk(direction) => self.walk(direction),
            Action::Wear(_) => "You can't do that yet.".to_owned(),
            Action::Where(noun) => self.where_is(noun),
        }
    }

    fn parse_move(&mut self, noun: &str) -> String {
        if let Some((loc, _)) = self
            .items
            .iter()
            .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(noun))
        {
            let room = self.item(&loc.to_owned()).location().to_owned();
            match self.item_mut(&loc.to_owned()).move_self() {
                Ok((message, reveals)) => {
                    if reveals.len() == 1 {
                        self.last_it = self.item(&reveals[0]).name().to_owned();
                    }
                    for x in reveals {
                        self.item_mut(&x).set_location(room.clone());
                    }
                    message
                }
                Err(err) => err,
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

        if self.item(&item).location() == self.player {
            if container == item {
                "Impossible.".to_owned()
            } else {
                match self.item(&container).container() {
                    Container::Open | Container::True => {
                        self.item_mut(&item).set_location(container);
                        "Done.".to_owned()
                    }
                    Container::Closed => {
                        self.last_it = self.item(&container).name().to_owned();
                        format!("The {} isn't open.", self.item(&container).name())
                    }
                    Container::False => "You can't do that.".to_owned(),
                }
            }
        } else {
            format!("You do not have the {}.", noun)
        }
    }

    fn replace_it(&self, tokens: Tokens) -> Tokens {
        if tokens.noun() == "it" {
            Tokens::with(
                tokens.verb().to_owned(),
                self.last_it.clone(),
                tokens.prep().to_owned(),
                tokens.obj().to_owned(),
            )
        } else {
            tokens
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
                zstd::stream::copy_encode(&*bincode::serialize(&self)?, file, 3)?;
                Ok("Saved.".to_owned())
            }
            Err(e) => Ok(e.to_string()),
        }
    }

    fn take_all(&mut self) -> String {
        let items = self.items.iter().fold(Vec::new(), |mut acc, (loc, i)| {
            if i.is_in(self.player_location())
                && !i.name().is_empty()
                && (i.can_take() || !i.take_message().is_empty())
            {
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

    fn update_last(&mut self, tokens: Tokens) {
        self.last_command = tokens;
        if !self.last_command.noun().is_empty() {
            self.last_it = self.last_command.noun().to_owned()
        }
    }

    fn walk(&mut self, direction: &str) -> String {
        if let Some((_, exit)) = self.items.iter().find(|(loc, i)| {
            self.is_visible(loc, i) && i.names_contains(direction) && !i.dest().is_empty()
        }) {
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
        } else if let Some((_, exit)) = self
            .items
            .iter()
            .find(|(loc, i)| self.is_visible(loc, i) && i.names_contains(direction))
        {
            if !exit.go_message().is_empty() {
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

    fn where_is(&self, noun: &str) -> String {
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
    }
}

fn cant_see_any(noun: &str) -> String {
    format!("You can't see any {} here.", noun)
}

fn prompt(p: &str) -> io::Result<String> {
    print!("{}", p);
    io::stdout().flush()?;
    read_line()
}
