use std::{collections::HashMap, mem, str::FromStr};

#[cfg(not(target_arch = "wasm32"))]
use std::{
    error,
    fs::File,
    io::{self, Write},
};

use serde::{Deserialize, Serialize};

use crate::{
    action::Action, container::Container, direction::Direction, item::Item, tokens::Tokens,
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
            list_names(
                &$items
                    .into_iter()
                    .map(|(_, i)| i.name())
                    .collect::<Vec<_>>(),
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
            list_names(
                &$items
                    .into_iter()
                    .map(|(_, i)| i.name())
                    .collect::<Vec<_>>(),
                "or"
            )
        );
    }};
}

macro_rules! find {
    ($self:ident, $verb:expr, $noun:ident, $message:expr) => {{
        let items = find_matches!($self, $noun, is_visible);

        match items.len() {
            0 => cant_see_any($noun),
            1 => return $message.to_owned(),
            _ => which!($self, $verb, $noun, items),
        }
    }};

    ($self:ident, $verb:expr, $noun:ident, $in:ident, $f:ident) => {{
        let items = find_matches!($self, $noun, $in);

        match items.len() {
            0 => cant_see_any($noun),
            1 => return $self.$f(&items[0].0.to_owned()),
            _ => which!($self, $verb, $noun, items),
        }
    }};

    ($self:ident, $verb:expr, $noun:ident, $in:ident, $obj:ident, $obj_in:ident, $f:ident) => {{
        if $noun == $obj {
            return "Impossible.".to_owned();
        } else {
            let noun_matches = find_matches!($self, $noun, $in);
            let obj_matches = find_matches!($self, $obj, $obj_in);

            match noun_matches.len() {
                0 => cant_see_any($noun),
                1 => match obj_matches.len() {
                    0 => cant_see_any($obj),
                    1 => {
                        return $self
                            .$f(&noun_matches[0].0.to_owned(), &obj_matches[0].0.to_owned())
                    }
                    _ => which!($self, $verb, $noun, $obj, obj_matches),
                },
                _ => which!($self, $verb, $noun, noun_matches),
            }
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
                    format!(
                        "{}\n{}: {}",
                        acc,
                        $self.item(loc).name().to_owned(),
                        $self.$f(loc)
                    )
                })
                .trim_start()
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
            .replace([',', ';', '.'], " and ")
            .split_whitespace()
            .map(str::to_lowercase)
            .fold(vec![Vec::new()], |mut acc, w| {
                if w == "and" || w == "then" {
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

                match self.parse(tokens.action()) {
                    Outcome::Active(output) => format!("{}{}", output, self.combat()),
                    Outcome::Idle(output) => output,
                }
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

                match self.parse(tokens.action()) {
                    Outcome::Active(output) => format!("{}{}", output, self.combat()),
                    Outcome::Idle(output) => output,
                }
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

                // If the verb isn't recognized, try to use the verb of the previous command,
                // but only if the previous command accepts a noun/obj
                // i.e.: "take apple and orange" would try "take apple and take orange"
                if !self.last_command.noun().is_empty() || !self.last_command.obj().is_empty() {
                    if let Action::Unknown(_) = tokens.action() {
                        tokens = Tokens::with(
                            self.last_command.verb().to_owned(),
                            words.join(" "),
                            self.last_command.prep().to_owned(),
                            self.last_command.obj().to_owned(),
                        );
                    }
                }

                self.update_last(tokens.clone());

                match self.parse(tokens.action()) {
                    Outcome::Active(output) => {
                        res = format!("{}\n\n{}{}", res, output, self.combat())
                    }
                    Outcome::Idle(output) => res = format!("{}\n\n{}", res, output,),
                }
            }
        }

        // wrap text
        let mut chunks: Vec<String> = Vec::new();
        for mut l in res.lines().map(str::to_owned) {
            loop {
                if l.len() < 80 {
                    chunks.push(mem::take(&mut l));
                } else if let Some(x) = l[..80].rfind(' ') {
                    chunks.push(l.drain(..x + 1).collect());
                } else if let Some(x) = l[80..].find(' ') {
                    chunks.push(l.drain(..x + 81).collect());
                } else {
                    chunks.push(mem::take(&mut l));
                };
                if l.is_empty() {
                    break;
                }
            }
        }
        chunks.join("\n")
    }

    fn combat(&mut self) -> String {
        let mut damage = 0;
        let res = self.items.iter().fold(String::new(), |acc, (_, i)| {
            if i.is_in(self.player_location()) && i.is_aggressive() {
                damage += i.damage();
                format!("{}\n\nThe {} hits you.", acc, i.name()) // TODO: random different messages
            } else {
                acc
            }
        });

        let player = self.item_mut(&self.player.to_owned());
        player.hurt(damage);

        if player.hp() <= 0 {
            format!("{}\n\nYou die.", res)
        } else {
            res
        }
    }

    fn attack(&mut self, enemy: &str, weapon: &str) -> String {
        let damage = self.item(weapon).damage();

        let enemy_item = self.item_mut(enemy);
        if enemy_item.hp() > 0 {
            enemy_item.hurt(damage);

            let enemy_name = enemy_item.name().to_owned();
            let dies = if self.item(enemy).hp() <= 0 {
                self.items.remove(enemy);

                let player_location = self.player_location().to_owned();
                let loot = self
                    .items
                    .values_mut()
                    .filter(|i| i.is_in(enemy))
                    .map(|i| {
                        i.set_location(player_location.clone());
                        i.name()
                    })
                    .collect::<Vec<_>>();

                if loot.is_empty() {
                    " It dies.".to_owned()
                } else {
                    if loot.len() == 1 {
                        self.last_it = loot[0].to_owned();
                    }
                    format!(" It dies. It drops {}.", list_names(&loot, "and"))
                }
            } else {
                String::new()
            };
            format!(
                "You hit the {} with your {}.{}",
                enemy_name,
                self.item(weapon).name(),
                dies
            )
        } else {
            format!("The {} has no effect.", self.item(weapon).name())
        }
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

        self.item_mut(location).set_location(player_location);
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
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load(filename: &str) -> Result<Self, Box<dyn error::Error>> {
        use std::fs;

        Ok(bincode::deserialize(&fs::read(filename)?)?)
    }

    // TODO: specific order?
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
            self.items.iter().fold(String::new(), |acc, (loc, i)| {
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
            })
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

    fn not_have(&mut self, location: &str) -> String {
        self.last_it = self.item(location).name().to_owned();
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
            .map(|i| i.name())
            .collect::<Vec<_>>();

        if self.item(&location).is_closed() && contents.len() == 1 {
            self.last_it = contents[0].to_owned();
        }
        let reveals = list_names(&contents, "and");

        self.item_mut(&location).open(reveals)
    }

    fn parse(&mut self, action: &Action) -> Outcome {
        match action {
            Action::Again => self.parse(&self.last_command.action().clone()),
            Action::Attack(noun, obj) => Outcome::Active(self.parse_attack(noun, obj)),
            Action::Break(_) => Outcome::Active("You can't do that yet.".to_owned()),
            Action::Burn(_, _) => Outcome::Active("You can't do that yet.".to_owned()),
            Action::Clarify(message) => Outcome::Idle(message.to_owned()),
            Action::Climb => Outcome::Active("You can't do that yet.".to_owned()),
            Action::Close(noun) => Outcome::Active(self.parse_close(noun)),
            Action::Drop(noun) => Outcome::Active(self.parse_drop(noun)),
            Action::Put(noun, obj) => Outcome::Active(self.parse_put(noun, obj)),
            Action::Eat(noun) => Outcome::Active(self.parse_eat(noun)),
            Action::Examine(noun) => Outcome::Active(self.parse_examine(noun)),
            Action::Hello => Outcome::Active("Hello!".to_owned()),
            Action::Help => Outcome::Idle("That would be nice, wouldn't it?".to_owned()),
            Action::Inventory => Outcome::Active(self.inventory()),
            Action::Look => Outcome::Active(self.look()),
            Action::Move(noun) => Outcome::Active(self.parse_move(noun)),
            Action::NoVerb => Outcome::Active("Excuse me?".to_owned()),
            Action::Open(noun) => Outcome::Active(self.parse_open(noun)),
            Action::Sleep => Outcome::Active("Time passes...".to_owned()),
            Action::Take(noun) => Outcome::Active(self.parse_take(noun)),
            Action::Unknown(verb) => Outcome::Idle(format!("I do not know the verb \"{}\".", verb)),
            Action::Version => Outcome::Idle(format!("Kingslayer {}", env!("CARGO_PKG_VERSION"))),
            Action::Walk(direction) => Outcome::Active(self.parse_walk(direction)),
            Action::Wear(_) => Outcome::Active("You can't do that yet.".to_owned()),
            Action::Where(noun) => Outcome::Active(self.parse_where(noun)),
        }
    }

    fn parse_attack(&mut self, noun: &str, obj: &str) -> String {
        find!(self, "attack", noun, in_room, obj, holding, attack);

        find!(self, "attack", obj, in_room, not_have);
        find!(self, "attack", noun, "You can't do that.")
    }

    fn parse_close(&mut self, noun: &str) -> String {
        find!(self, "close", noun, is_visible, close)
    }

    fn parse_drop(&mut self, noun: &str) -> String {
        do_all!(self, "drop", noun, in_inventory, drop_item);

        find!(self, "drop", noun, in_inventory, drop_item);

        find!(self, "drop", noun, is_visible, not_have)
    }

    fn parse_eat(&mut self, noun: &str) -> String {
        find!(self, "eat", noun, is_visible, eat)
    }

    fn parse_examine(&mut self, noun: &str) -> String {
        find!(self, "examine", noun, is_visible_has_details, examine);
        find!(self, "examine", noun, is_visible_has_door, examine_door);

        find!(self, "examine", noun, is_visible, examine_container)
    }

    fn parse_move(&mut self, noun: &str) -> String {
        find!(self, "move", noun, is_visible, move_item)
    }

    fn parse_open(&mut self, noun: &str) -> String {
        find!(self, "open", noun, is_visible, open)
    }

    fn parse_put(&mut self, noun: &str, obj: &str) -> String {
        find!(self, "put", noun, in_inventory, obj, is_visible, put);

        find!(self, "put", noun, is_visible, not_have)
    }

    fn parse_take(&mut self, noun: &str) -> String {
        do_all!(self, "take", noun, is_visible_not_holding, take);

        find!(self, "take", noun, in_room, take);
        find!(self, "take", noun, is_visible_not_holding, take);
        find!(self, "take", noun, in_inventory, have_already);

        find!(self, "take", noun, is_visible, take)
    }

    fn parse_walk(&mut self, direction: &str) -> String {
        find!(self, "go", direction, is_visible_has_dest, walk);
        find!(self, "go", direction, is_visible, walk_fail);

        if direction.is_direction() || direction == "enter" {
            "You cannot go that way.".to_owned()
        } else {
            cant_see_any(direction)
        }
    }

    fn parse_where(&mut self, noun: &str) -> String {
        if noun == "i" {
            self.item(self.player_location()).desc().to_owned()
        } else {
            find!(self, "where", noun, "It's here.")
        }
    }

    /// Start the Game in a command line setting where `print` macros are expected to work
    #[cfg(not(target_arch = "wasm32"))]
    pub fn play(&mut self) -> Result<(), Box<dyn error::Error>> {
        println!("{}", self.ask("l"));

        loop {
            println!(
                "{}",
                match prompt("\n> ")?.trim() {
                    "quit" | "q" =>
                        if confirm_quit()? {
                            break;
                        } else {
                            "Ok.".to_owned()
                        },
                    "restore" => self.restore("kingslayer.save"), // TODO: ask for filename
                    "save" => self.save("kingslayer.save")?,      // TODO: ask for filename
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
        match self.item(container).container() {
            Container::Open | Container::True => {
                self.item_mut(item).set_location(container.to_owned());
                "Done.".to_owned()
            }
            Container::Closed => {
                self.last_it = self.item(container).name().to_owned();
                format!("The {} isn't open.", self.item(container).name())
            }
            Container::False => "You can't do that.".to_owned(),
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
    #[cfg(not(target_arch = "wasm32"))]
    pub fn restore(&mut self, filename: &str) -> String {
        match Game::load(filename) {
            Ok(game) => {
                self.player = game.player;
                self.items = game.items;

                "OK".to_owned()
            }
            Err(e) => e.to_string(),
        }
    }

    /// Save the Game to a file.
    /// ```
    /// # use kingslayer::Game;
    /// # let game = Game::default();
    /// game.save("kingslayer.save");
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&self, filename: &str) -> Result<String, Box<dyn error::Error>> {
        Ok(match File::create(filename) {
            Ok(mut file) => {
                file.write_all(&bincode::serialize(self)?)?;
                "Saved.".to_owned()
            }
            Err(e) => e.to_string(),
        })
    }

    fn take(&mut self, location: &str) -> String {
        self.items
            .get_mut(location)
            .unwrap()
            .take(&self.player)
            .to_owned()
    }

    fn update_last(&mut self, tokens: Tokens) {
        self.last_command = tokens;
        if !self.last_command.noun().is_empty() && self.last_command.noun() != "all" {
            self.last_it = self.last_command.noun().to_owned()
        }
    }

    // TODO: prevent if in combat
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
        self.last_it = self.item(location).name().to_owned();
        self.item(location).go_message().to_owned()
    }
}

fn cant_see_any(noun: &str) -> String {
    format!("You can't see any {} here.", noun)
}

fn list_names(names: &[&str], sep: &str) -> String {
    let a = if sep == "or" { "the" } else { "a" };

    match names.len() {
        0 => String::new(),
        1 => format!("{a} {}", names[0]),
        2 => format!("{a} {} {sep} {a} {}", names[0], names[1]),
        _ => {
            format!(
                "{a} {}, {sep} {a} {}",
                names[1..names.len() - 1]
                    .iter()
                    .fold(names[0].to_owned(), |acc, i| format!("{}, {a} {}", acc, i)),
                names[names.len() - 1]
            )
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn prompt(message: &str) -> io::Result<String> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

#[cfg(not(target_arch = "wasm32"))]
fn confirm_quit() -> io::Result<bool> {
    let res = prompt("Are you sure you want to quit? (y/n): ")?
        .trim()
        .to_lowercase();

    Ok(res == "y" || res == "yes" || res.starts_with("y ") || res.starts_with("yes "))
}

enum Outcome {
    Active(String),
    Idle(String),
}
