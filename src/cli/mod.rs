use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::errors::WorldError;
use crate::item::Item;
use crate::player::Player;
use crate::utils::read_line;
use crate::world::World;

// A command line interface for controlling interactions between objects in a game
#[derive(Serialize, Deserialize)]
pub struct Cli {
    player: RefCell<Player>,
    world: RefCell<World>,
}

impl Cli {
    // handle user input
    pub fn ask(&self, input: &str) -> String {
        let filter_out = ["a", "an", "at", "go", "my", "of", "that", "the", "to"];

        let mut command = self.parts(input);
        command.retain(|w| !(&filter_out).contains(&w.as_str()));
        let command = self.mod_directions(&command);

        if !command.is_empty() {
            format!("{}{}", self.parse(&command), self.events().unwrap())
        } else {
            "I do not understand that phrase.".to_string()
        }
    }

    // prompts the user for an action from stdin
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
    fn parse(&self, words: &[String]) -> String {
        match words[0].as_str() {
            "l" | "look" => self.world.borrow().look().unwrap(),
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                self.world.borrow_mut().move_room(&words[0]).unwrap()
            }
            "enter" => {
                if words.len() > 1 {
                    self.world.borrow_mut().move_room(&words[1]).unwrap()
                } else {
                    format!("Where do you want to {}?", words[0])
                }
            }
            "i" | "inventory" => self.player.borrow().inventory(),
            "take" | "get" | "pick" => {
                if words.len() > 1 {
                    if let Some(pos) = words
                        .iter()
                        .position(|r| r == "from" || r == "out" || r == "in")
                    {
                        if self
                            .player
                            .borrow()
                            .inventory()
                            .contains(&words[pos + 1..].join(" "))
                        {
                            self.player
                                .borrow_mut()
                                .take_from(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                        } else {
                            self.player.borrow_mut().take(
                                &words[1..pos].join(" "),
                                self.world.borrow_mut().give_from(
                                    &words[1..pos].join(" "),
                                    &words[pos + 1..].join(" "),
                                ),
                            )
                        }
                    } else if words[1] == "all" {
                        self.player
                            .borrow_mut()
                            .take_all(self.world.borrow_mut().give_all())
                    } else if &words[1] == "u" {
                        self.player.borrow_mut().take(
                            &words[2..].join(" "),
                            self.world.borrow_mut().give(&words[2..].join(" ")),
                        )
                    } else {
                        self.player.borrow_mut().take(
                            &words[1..].join(" "),
                            self.world.borrow_mut().give(&words[1..].join(" ")),
                        )
                    }
                } else {
                    format!("What do you want to {}?", words[0])
                }
            }
            "drop" | "throw" | "remove" => {
                if words.len() > 1 {
                    self.world
                        .borrow_mut()
                        .insert(
                            &words[0],
                            &words[1..].join(" "),
                            self.player.borrow_mut().remove(&words[1..].join(" ")),
                        )
                        .unwrap()
                } else {
                    format!("What do you want to {} from your inventory?", words[0])
                }
            }
            "examine" | "inspect" | "read" => {
                if words.len() > 1 {
                    if let Some(s) = self.player.borrow().inspect(&words[1..].join(" ")) {
                        s
                    } else if let Some(s) = self.world.borrow().inspect(&words[1..].join(" ")) {
                        s
                    } else {
                        format!("There is no \"{}\" here.", &words[1..].join(" "))
                    }
                } else {
                    format!("What do you want to {}?", words[0])
                }
            }
            "status" | "diagnostic" => self.player.borrow().status(),
            "put" | "place" => {
                if words.len() > 1 {
                    if let Some(pos) = words.iter().position(|r| r == "in" || r == "inside") {
                        if pos != 1 {
                            if self
                                .player
                                .borrow()
                                .inventory()
                                .contains(&words[pos + 1..].join(" "))
                            {
                                self.player
                                    .borrow_mut()
                                    .put_in(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                            } else {
                                self.world
                                    .borrow_mut()
                                    .insert_into(
                                        &words[1..pos].join(" "),
                                        &words[pos + 1..].join(" "),
                                        self.player.borrow_mut().remove(&words[1..pos].join(" ")),
                                    )
                                    .unwrap()
                            }
                        } else if words.len() < 3 {
                            format!("What do you want to {}?", words[0])
                        } else {
                            format!(
                                "What do you want to place in the {}?",
                                &words[1..].join(" ")
                            )
                        }
                    } else {
                        format!(
                            "What do you want to {} the {} in?",
                            words[0],
                            &words[1..].join(" ")
                        )
                    }
                } else {
                    format!("What do you want to {}?", words[0])
                }
            }
            "attack" | "slay" | "kill" | "hit" => {
                if words.len() > 1 {
                    if let Some(pos) = words.iter().position(|r| r == "with") {
                        let damage = self
                            .player
                            .borrow_mut()
                            .attack_with(&words[pos + 1..].join(" "));
                        self.world
                            .borrow_mut()
                            .harm_enemy(
                                &words[1..pos].join(" "),
                                &words[pos + 1..].join(" "),
                                damage,
                            )
                            .unwrap()
                    } else if self.player.borrow().main_hand.is_some() {
                        let damage = self.player.borrow_mut().attack();
                        self.world
                            .borrow_mut()
                            .harm_enemy(&words[1..].join(" "), "equipped weapon", damage)
                            .unwrap()
                    } else {
                        format!(
                            "What do you want to {} the {} with?",
                            words[0],
                            &words[1..].join(" ")
                        )
                    }
                } else {
                    format!("What do you want to {}?", words[0])
                }
            }
            "rest" | "sleep" | "heal" => {
                if !self.player.borrow().in_combat {
                    self.player.borrow_mut().rest()
                } else {
                    "You cannot rest while in combat.".to_string()
                }
            }
            "hold" | "draw" | "equip" => {
                if words.len() > 1 {
                    self.player.borrow_mut().equip(&words[1..].join(" "))
                } else {
                    format!("What do you want to {}", words[0])
                }
            }
            _ => format!("I don't know the word \"{}\".", words[0]),
        }
    }

    // computes actions taken by Enemies in the current room
    fn events(&self) -> Result<String, WorldError> {
        let curr_room = &self.world.borrow().curr_room();

        if let Some(room) = self.world.borrow_mut().rooms.get_mut(curr_room) {
            let mut events_str = String::new();
            let mut dropped_loot: HashMap<String, Box<Item>> = HashMap::new();
            for enemy in room.enemies.iter_mut() {
                if enemy.1.is_angry() && enemy.1.hp() > 0 {
                    let enemy_damage = enemy.1.damage();

                    self.player.borrow_mut().take_damage(enemy_damage);
                    self.player.borrow_mut().in_combat = true;

                    events_str.push_str(&format!(
                        "\nThe {} hit you for {} damage. You have {} HP left.",
                        enemy.1.name(),
                        enemy_damage,
                        self.player.borrow().hp()
                    ));
                }
                if enemy.1.hp() <= 0 {
                    self.player.borrow_mut().in_combat = false;
                    dropped_loot.extend(enemy.1.drop_loot());
                }
            }
            room.items.extend(dropped_loot);
            room.enemies.retain(|_, e| e.hp() > 0);
            if self.player.borrow().hp() <= 0 {
                events_str.push_str(" You died.");
            }
            Ok(events_str)
        } else {
            Err(WorldError::NoRoom)
        }
    }
}
