use std::collections::HashMap;
use std::io::{self, Write};

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::item::Item;
use crate::player::Player;
use crate::utils::read_line;
use crate::world::World;

// A command line interface for controlling interactions between objects in a game
#[derive(Serialize, Deserialize)]
pub struct Cli {
    player: Player,
    world: World,
}

impl Cli {
    pub fn ask(&mut self, input: &str) -> String {
        let filter_out = ["a", "an", "at", "go", "my", "of", "that", "the", "to"];

        let mut command = self.parts(input);
        command.retain(|w| !(&filter_out).contains(&w.as_str()));
        let command = self.mod_directions(&command);

        if !command.is_empty() {
            format!("{}{}", self.parse(&command), self.events())
        } else {
            "I do not understand that phrase.".to_string()
        }
    }

    // prompts the user for an action
    pub fn prompt(&self) -> String {
        loop {
            print!("\n> ");
            io::stdout().flush().expect("error flushing");
            let input = read_line();
            if !input.is_empty() {
                return input;
            } else {
                println!("Excuse me?");
            }
        }
    }

    // splits a string into a vector of individual words
    fn parts(&self, s: &str) -> Vec<String> {
        s.split_whitespace()
            .map(|x| x.to_lowercase().to_string())
            .collect()
    }

    // modify path directives
    fn mod_directions(&self, words: &[String]) -> Vec<String> {
        let mut modified = Vec::with_capacity(words.len());
        for w in words {
            match w.as_str() {
                "north" => modified.push("n".to_string()),
                "south" => modified.push("s".to_string()),
                "east" => modified.push("e".to_string()),
                "west" => modified.push("w".to_string()),
                "northeast" => modified.push("ne".to_string()),
                "northwest" => modified.push("nw".to_string()),
                "southeast" => modified.push("se".to_string()),
                "southwest" => modified.push("sw".to_string()),
                "up" => modified.push("u".to_string()),
                "down" => modified.push("d".to_string()),
                _ => modified.push(w.to_string()),
            }
        }
        modified
    }

    // interprets words as game commands
    fn parse(&mut self, words: &[String]) -> String {
        match words[0].as_str() {
            "l" | "look" => self.world.look(),
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                self.world.move_room(&words[0])
            }
            "enter" => {
                if words.len() > 1 {
                    self.world.move_room(&words[1])
                } else {
                    format!("Where do you want to {}?", &words[0])
                }
            }
            "i" | "inventory" => self.player.inventory(),
            "take" | "get" | "pick" => {
                if words.len() > 1 {
                    match words
                        .iter()
                        .position(|r| r == "from" || r == "out" || r == "in")
                    {
                        Some(pos) => self.player.take(
                            &words[1..pos].join(" "),
                            self.world
                                .give_from(&words[1..pos].join(" "), &words[pos + 1..].join(" ")),
                        ),
                        None => {
                            if words[1] == "all" {
                                self.player.take_all(self.world.give_all())
                            } else if &words[1] == "u" {
                                self.player.take(
                                    &words[2..].join(" "),
                                    self.world.give(&words[2..].join(" ")),
                                )
                            } else {
                                self.player.take(
                                    &words[1..].join(" "),
                                    self.world.give(&words[1..].join(" ")),
                                )
                            }
                        }
                    }
                } else {
                    format!("What do you want to {}?", &words[0])
                }
            }
            "drop" | "throw" | "remove" => {
                if words.len() > 1 {
                    self.world.insert(
                        &words[0],
                        &words[1..].join(" "),
                        self.player.remove(&words[1..].join(" ")),
                    )
                } else {
                    format!("What do you want to {} from your inventory?", &words[0])
                }
            }
            "examine" | "inspect" | "read" => {
                if words.len() > 1 {
                    match self.player.inspect(&words[1..].join(" ")) {
                        Some(s) => s,
                        None => match self.world.inspect(&words[1..].join(" ")) {
                            Some(s) => s,
                            None => format!("There is no \"{}\" here.", &words[1..].join(" ")),
                        },
                    }
                } else {
                    format!("What do you want to {}?", &words[0])
                }
            }
            "status" | "diagnostic" => self.player.status(),
            "put" | "place" => {
                if words.len() > 1 {
                    match words.iter().position(|r| r == "in" || r == "inside") {
                        Some(pos) => {
                            if pos != 1 {
                                self.world.insert_into(
                                    &words[1..pos].join(" "),
                                    &words[pos + 1..].join(" "),
                                    self.player.remove(&words[1..pos].join(" ")),
                                )
                            } else if words.len() < 3 {
                                format!("What do you want to {}?", &words[0])
                            } else {
                                format!(
                                    "What do you want to place in the {}?",
                                    &words[1..].join(" ")
                                )
                            }
                        }
                        None => format!(
                            "What do you want to {} the {} in?",
                            &words[0],
                            &words[1..].join(" ")
                        ),
                    }
                } else {
                    format!("What do you want to {}?", &words[0])
                }
            }
            "attack" | "slay" | "kill" | "hit" => {
                if words.len() > 1 {
                    match words.iter().position(|r| r == "with") {
                        Some(pos) => {
                            let damage = self.player.attack(&words[pos + 1..].join(" "));
                            self.world.harm_enemy(
                                &words[1..pos].join(" "),
                                &words[pos + 1..].join(" "),
                                damage,
                            )
                        }
                        None => format!(
                            "What do you want to {} the {} with?",
                            &words[0],
                            &words[1..].join(" ")
                        ),
                    }
                } else {
                    format!("What do you want to {}?", &words[0])
                }
            }
            "rest" | "sleep" | "heal" => {
                if !self.player.in_combat {
                    self.player.rest()
                } else {
                    "You cannot rest while in combat.".to_string()
                }
            }
            "equip" => {
                if words.len() > 1 {
                    self.player.equip(&words[1..].join(" "))
                } else {
                    format!("What do you want to {}", &words[0])
                }
            }
            _ => format!("I don't know the word \"{}\".", &words[0]),
        }
    }

    // computes actions taken by Enemies in the current room
    fn events(&mut self) -> String {
        let curr_room = &self.world.curr_room();

        match self.world.rooms.get_mut(curr_room) {
            Some(room) => {
                let mut events_str = String::new();
                let mut dropped_loot: HashMap<String, Box<Item>> = HashMap::new();
                for enemy in room.enemies.iter_mut() {
                    if enemy.1.is_angry() && enemy.1.hp() > 0 {
                        let enemy_damage = enemy.1.damage();

                        self.player.take_damage(enemy_damage);
                        self.player.in_combat = true;

                        events_str.push_str(&format!(
                            "\nThe {} hit you for {} damage. You have {} HP left.",
                            enemy.1.name(),
                            enemy_damage,
                            self.player.hp()
                        ));
                    }
                    if enemy.1.hp() <= 0 {
                        self.player.in_combat = false;
                        dropped_loot.extend(enemy.1.drop_loot());
                    }
                }
                room.items.extend(dropped_loot);
                room.enemies.retain(|_, e| e.hp() > 0);
                if self.player.hp() <= 0 {
                    events_str.push_str(" You died.");
                }
                events_str
            }
            None => "You are not in a room...".to_string(),
        }
    }
}
