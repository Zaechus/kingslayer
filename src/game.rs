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

macro_rules! find_matches {
    ($self:ident, $noun:ident, $in:ident) => {
        $self
            .items
            .iter()
            .filter(|(_, i)| $self.$in(i) && i.names_contains($noun))
            .collect::<Vec<_>>()
    };
}

macro_rules! which {
    ($self:ident, $verb:expr, $noun:expr, $items:expr) => {{
        $self.last_command = Tokens::with(
            $verb.to_owned(),
            String::new(),
            $self.last_command.prep().to_owned(),
            $self.last_command.obj().to_owned(),
        );
        return format!(
            "Which {}, {}?",
            $noun,
            list_items(
                &$items.into_iter().map(|(_, i)| i).collect::<Vec<_>>(),
                "or"
            )
        );
    }};

    ($self:ident, $verb:expr, $noun:expr, $obj:expr, $items:expr) => {{
        $self.last_command = Tokens::with(
            $verb.to_owned(),
            $noun.to_owned(),
            $self.last_command.prep().to_owned(),
            String::new(),
        );
        return format!(
            "Which {}, {}?",
            $obj,
            list_items(
                &$items.into_iter().map(|(_, i)| i).collect::<Vec<_>>(),
                "or"
            )
        );
    }};
}

macro_rules! find {
    ($self:ident, $verb:expr, $noun:ident, $f:ident) => {{
        let items = find_matches!($self, $noun, is_visible);

        match items.len() {
            0 => cant_see_any($noun),
            1 => $self.$f(&items[0].0.to_owned()),
            _ => which!($self, $verb, $noun, items),
        }
    }};

    ($self:ident, $verb:expr, $noun:ident, $message:expr) => {{
        let items = find_matches!($self, $noun, is_visible);

        match items.len() {
            0 => cant_see_any($noun),
            1 => $message.to_owned(),
            _ => which!($self, $verb, $noun, items),
        }
    }};
}

macro_rules! try_find {
    ($self:ident, $verb:expr, $noun:ident, $in:ident, $f:ident) => {{
        let items = find_matches!($self, $noun, $in);

        match items.len() {
            0 => (),
            1 => return $self.$f(&items[0].0.to_owned()),
            _ => which!($self, $verb, $noun, items),
        }
    }};

    ($self:ident, $verb:expr, $noun:ident, $in:ident, $obj:ident, $f:ident) => {{
        let noun_matches = find_matches!($self, $noun, $in);
        let obj_matches = find_matches!($self, $obj, is_visible);

        match noun_matches.len() {
            0 => cant_see_any($noun),
            1 => match obj_matches.len() {
                0 => return cant_see_any($obj),
                1 => return $self.$f(&noun_matches[0].0.to_owned(), &obj_matches[0].0.to_owned()),
                _ => which!($self, $verb, $noun, $obj, obj_matches),
            },
            _ => which!($self, $verb, $noun, noun_matches),
        }
    }};
}

macro_rules! do_all {
    ($self:ident, $verb:expr, $noun:expr, $in:ident, $f:ident) => {{
        if $noun == "all" {
            let items = $self
                .items
                .iter()
                .filter(|(_, i)| $self.$in(i) && !i.name().is_empty() && i.try_take())
                .map(|(loc, _)| loc.to_owned())
                .collect::<Vec<_>>();

            let message = items
                .iter()
                .fold(String::new(), |acc, loc| {
                    let name = $self.item(loc).name().to_owned();
                    format!("{}\n{}: {}", acc, name, $self.$f(loc))
                })
                .trim()
                .to_owned();

            return if message.is_empty() {
                format!("You can't see anything you can {}.", $verb)
            } else {
                message
            };
        }
    }};
}

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
                // answer question unless issuing a known command
                if let Action::Unknown(_) = tokens.action() {
                    let words = if tokens.verb() == "it" {
                        self.last_it.clone()
                    } else {
                        first.join(" ")
                    };

                    // attempt to fill noun and then obj
                    if self.last_command.noun().is_empty() {
                        tokens = Tokens::with(
                            self.last_command.verb().to_owned(),
                            words,
                            self.last_command.prep().to_owned(),
                            self.last_command.obj().to_owned(),
                        );
                    } else {
                        tokens = Tokens::with(
                            self.last_command.verb().to_owned(),
                            self.last_command.noun().to_owned(),
                            self.last_command.prep().to_owned(),
                            words,
                        );
                    }
                }

                self.update_last(tokens.clone());

                self.parse(tokens.action())
            } else {
                let tokens = self.replace_it(tokens);

                // don't update last_it if current action is a question
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

        // all secondary commands
        for words in commands.iter().skip(1) {
            // do not continue if last parsed command was a question
            if let Action::Clarify(_) = self.last_command.action() {
                break;
            } else {
                let mut tokens = self.replace_it(Tokens::new(words));

                // if the verb isn't recognized, try to use the verb of the previous command
                // i.e.: "take apple and orange" would try "take apple and take orange"
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

        // wrap text
        let mut chunks: Vec<String> = Vec::new();
        for mut l in res.lines().map(str::to_owned) {
            loop {
                if l.len() < 80 {
                    chunks.push(l.drain(..).collect());
                } else if let Some(x) = l[..80].rfind(' ') {
                    chunks.push(l.drain(..x + 1).collect());
                } else if let Some(x) = l[80..].find(' ') {
                    chunks.push(l.drain(..x + 81).collect());
                } else {
                    chunks.push(l.drain(..).collect());
                };
                if l.is_empty() {
                    break;
                }
            }
        }
        chunks.join("\n")
    }

    fn close(&mut self, location: &str) -> String {
        let item = self.item(location);
        let location = if !item.door().is_empty() {
            item.door().to_owned()
        } else {
            location.to_owned()
        };

        self.item_mut(&location).close()
    }

    // print the contents of an item
    fn contents(&self, location: &str, item: &Item, depth: usize) -> String {
        if item.is_clear() {
            let contents = self.items.iter().fold(String::new(), |acc, (loc, i)| {
                if i.is_in(location) && !i.name().is_empty() {
                    let contents = if i.is_container() {
                        self.contents(loc, i, depth + 1)
                    } else {
                        String::new()
                    };

                    if contents.is_empty() {
                        format!("{}\n{}a {}", acc, "  ".repeat(depth), i.name())
                    } else {
                        format!(
                            "{}\n{}a {}\n{}",
                            acc,
                            "  ".repeat(depth),
                            i.name(),
                            contents
                        )
                    }
                } else {
                    acc
                }
            });

            if contents.is_empty() {
                String::new()
            } else {
                format!(
                    "{}The {} contains:{}",
                    "  ".repeat(depth - 1),
                    item.name(),
                    contents
                )
            }
        } else {
            String::new()
        }
    }

    // print self.contents(...) with the item desc
    fn desc_contents(&self, location: &str, item: &Item) -> String {
        let contents = self.contents(location, item, 1);

        if contents.is_empty() {
            item.desc().to_owned()
        } else if item.desc().is_empty() {
            contents
        } else {
            format!("{}\n{}", item.desc(), contents)
        }
    }

    fn drop_item(&mut self, location: &str) -> String {
        let player_location = self.player_location().to_owned();

        self.item_mut(&location).set_location(player_location);
        "Dropped.".to_owned()
    }

    fn eat(&mut self, location: &str) -> String {
        if self.item(location).can_eat() {
            self.items.remove(location);
            "Delicious.".to_owned()
        } else {
            "You cannot eat that.".to_owned()
        }
    }

    fn examine(&self, location: &str) -> String {
        self.item(location).details().to_owned()
    }

    fn examine_container(&self, location: &str) -> String {
        let item = self.item(location);

        match item.container() {
            Container::Open | Container::True => {
                let contents = self.contents(location, item, 1);

                if contents.is_empty() {
                    format!("The {} is empty.", item.name())
                } else {
                    contents
                }
            }
            Container::Closed => format!("The {} is closed.", item.name()),
            _ => format!("There is nothing remarkable about the {}.", item.name()),
        }
    }

    fn examine_door(&self, location: &str) -> String {
        let item = self.item(self.item(location).door());

        match item.container() {
            Container::Open | Container::True => format!("The {} is open.", item.name()),
            Container::Closed => format!("The {} is closed.", item.name()),
            _ => format!("There is nothing remarkable about the {}.", item.name()),
        }
    }

    fn have_already(&self, location: &str) -> String {
        format!("You already have the {}.", self.item(location).name())
    }

    fn holding(&self, item: &Item) -> bool {
        item.is_in(&self.player)
    }

    fn in_inventory(&self, item: &Item) -> bool {
        self.item_in(item, &self.player)
    }

    fn in_room(&self, item: &Item) -> bool {
        self.item_in(item, self.player_location())
    }

    fn inventory(&self) -> String {
        let inv = self.items.iter().fold(String::new(), |acc, (loc, i)| {
            if i.is_in(&self.player) {
                let contents = if i.is_container() {
                    self.contents(loc, i, 2)
                } else {
                    String::new()
                };

                if contents.is_empty() {
                    format!("{}\n  a {}", acc, i.name())
                } else {
                    format!("{}\n  a {}\n{}", acc, i.name(), contents)
                }
            } else {
                acc
            }
        });

        if inv.is_empty() {
            "Your inventory is empty.".to_owned()
        } else {
            format!("You are carrying:{}", inv)
        }
    }

    // is the item visible in the room or in inventory
    fn is_visible(&self, item: &Item) -> bool {
        self.in_inventory(item) || self.in_room(item)
    }

    fn is_visible_has_dest(&self, item: &Item) -> bool {
        self.is_visible(item) && !item.dest().is_empty()
    }

    fn is_visible_has_details(&self, item: &Item) -> bool {
        self.is_visible(item) && !item.details().is_empty()
    }

    fn is_visible_has_door(&self, item: &Item) -> bool {
        self.is_visible(item) && !item.door().is_empty()
    }

    // is the item visible in the room or held by the player
    fn is_visible_not_holding(&self, item: &Item) -> bool {
        (self.in_inventory(item) || self.in_room(item)) && !self.holding(item)
    }

    fn item(&self, key: &str) -> &Item {
        self.items.get(key).unwrap()
    }

    // TODO: recursion?
    fn item_in(&self, item: &Item, location: &str) -> bool {
        item.is_in(location)
            || if let Some(parent) = self.items.get(item.location()) {
                parent.is_open()
                    && (parent.is_in(location)
                        || if let Some(super_parent) = self.items.get(parent.location()) {
                            super_parent.is_open() && super_parent.is_in(location)
                        } else {
                            false
                        })
            } else {
                false
            }
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

    fn move_item(&mut self, location: &str) -> String {
        let room = self.item(location).location().to_owned();

        match self.item_mut(location).move_self() {
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
    }

    fn not_have(&self, location: &str) -> String {
        format!("You do not have the {}.", self.item(location).name())
    }

    fn open(&mut self, location: &str) -> String {
        let item = self.item(location);
        let location = if !item.door().is_empty() {
            item.door().to_owned()
        } else {
            location.to_owned()
        };

        let contents = self
            .items
            .values()
            .filter(|i| i.is_in(&location))
            .collect::<Vec<_>>();

        if self.item(&location).is_closed() && contents.len() == 1 {
            self.last_it = contents[0].name().to_owned();
        }
        let reveals = list_items(&contents, "and");

        self.item_mut(&location).open(reveals)
    }

    fn parse(&mut self, action: &Action) -> String {
        match action {
            Action::Again => self.parse(&self.last_command.action().clone()),
            Action::Attack(_, _) => "You can't do that yet.".to_owned(),
            Action::Break(_) => "You can't do that yet.".to_owned(),
            Action::Burn(_, _) => "You can't do that yet.".to_owned(),
            Action::Clarify(message) => message.to_owned(),
            Action::Climb => "You can't do that yet.".to_owned(),
            Action::Close(noun) => self.parse_close(noun),
            Action::Drop(noun) => self.parse_drop(noun),
            Action::Put(noun, obj) => self.parse_put(noun, obj),
            Action::Eat(noun) => self.parse_eat(noun),
            Action::Examine(noun) => self.parse_examine(noun),
            Action::Hello => "Hello!".to_owned(),
            Action::Help => "That would be nice, wouldn't it?".to_owned(),
            Action::Inventory => self.inventory(),
            Action::Look => self.look(),
            Action::Move(noun) => self.parse_move(noun),
            Action::NoVerb => "Excuse me?".to_owned(),
            Action::Open(noun) => self.parse_open(noun),
            Action::Sleep => "Time passes...".to_owned(),
            Action::Take(noun) => self.parse_take(noun),
            Action::Unknown(verb) => format!("I do not know the verb \"{}\".", verb),
            Action::Version => format!("Kingslayer {}", env!("CARGO_PKG_VERSION")),
            Action::Walk(direction) => self.parse_walk(direction),
            Action::Wear(_) => "You can't do that yet.".to_owned(),
            Action::Where(noun) => self.parse_where(noun),
        }
    }

    fn parse_close(&mut self, noun: &str) -> String {
        find!(self, "close", noun, close)
    }

    fn parse_drop(&mut self, noun: &str) -> String {
        do_all!(self, "drop", noun, in_inventory, drop_item);

        try_find!(self, "drop", noun, in_inventory, drop_item);

        find!(self, "drop", noun, not_have)
    }

    fn parse_eat(&mut self, noun: &str) -> String {
        find!(self, "eat", noun, eat)
    }

    fn parse_examine(&mut self, noun: &str) -> String {
        try_find!(self, "examine", noun, is_visible_has_details, examine);
        try_find!(self, "examine", noun, is_visible_has_door, examine_door);

        find!(self, "examine", noun, examine_container)
    }

    fn parse_move(&mut self, noun: &str) -> String {
        find!(self, "move", noun, move_item)
    }

    fn parse_open(&mut self, noun: &str) -> String {
        find!(self, "open", noun, open)
    }

    fn parse_put(&mut self, noun: &str, obj: &str) -> String {
        try_find!(self, "put", noun, in_inventory, obj, put);

        find!(self, "put", noun, not_have)
    }

    fn parse_take(&mut self, noun: &str) -> String {
        do_all!(self, "take", noun, is_visible_not_holding, take_item);

        try_find!(self, "take", noun, in_room, take_item);
        try_find!(self, "take", noun, is_visible_not_holding, take_item);
        try_find!(self, "take", noun, in_inventory, have_already);

        find!(self, "take", noun, take_item)
    }

    fn parse_walk(&mut self, direction: &str) -> String {
        try_find!(self, "go", direction, is_visible_has_dest, walk);
        try_find!(self, "go", direction, is_visible, walk_fail);

        if direction.is_direction() || direction == "enter" {
            "You cannot go that way.".to_owned()
        } else {
            cant_see_any(direction)
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

    fn put(&mut self, item: &str, container: &str) -> String {
        if container == item {
            "Impossible.".to_owned()
        } else {
            match self.item(&container).container() {
                Container::Open | Container::True => {
                    self.item_mut(&item).set_location(container.to_owned());
                    "Done.".to_owned()
                }
                Container::Closed => {
                    self.last_it = self.item(&container).name().to_owned();
                    format!("The {} isn't open.", self.item(&container).name())
                }
                Container::False => "You can't do that.".to_owned(),
            }
        }
    }

    // replace the noun "it" (or "them") in a Tokens with the last referenced object
    fn replace_it(&self, tokens: Tokens) -> Tokens {
        match (tokens.noun(), tokens.obj()) {
            ("it", "it") => Tokens::with(
                tokens.verb().to_owned(),
                self.last_it.clone(),
                tokens.prep().to_owned(),
                self.last_it.clone(),
            ),
            ("it", _) => Tokens::with(
                tokens.verb().to_owned(),
                self.last_it.clone(),
                tokens.prep().to_owned(),
                tokens.obj().to_owned(),
            ),
            (_, "it") => Tokens::with(
                tokens.verb().to_owned(),
                tokens.noun().to_owned(),
                tokens.prep().to_owned(),
                self.last_it.clone(),
            ),
            _ => tokens,
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

    fn take_item(&mut self, location: &str) -> String {
        self.items
            .get_mut(location)
            .unwrap()
            .take(&self.player)
            .to_owned()
    }

    fn update_last(&mut self, tokens: Tokens) {
        self.last_command = tokens;
        if !self.last_command.noun().is_empty() {
            self.last_it = self.last_command.noun().to_owned()
        }
    }

    fn walk(&mut self, location: &str) -> String {
        let exit = self.item(location);
        let exit_dest = exit.dest().to_owned();

        if let Some(door) = self.items.get(exit.door()) {
            if door.is_open() {
                self.item_mut(&self.player.clone()).set_location(exit_dest);
                self.look()
            } else {
                self.last_it = door.name().to_owned();
                format!("The {} is closed.", door.name())
            }
        } else {
            self.item_mut(&self.player.clone()).set_location(exit_dest);
            self.look()
        }
    }

    fn walk_fail(&mut self, location: &str) -> String {
        let exit = self.item(location);

        if !exit.go_message().is_empty() {
            exit.go_message().to_owned()
        } else {
            "Nice try.".to_owned()
        }
    }

    fn parse_where(&mut self, noun: &str) -> String {
        if noun == "i" {
            self.item(self.player_location()).desc().to_owned()
        } else {
            find!(self, "where", noun, "It's here.")
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
