use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufReader, Read};

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::errors::WorldError;
use crate::item::Item;
use crate::player::Player;
use crate::results::CmdResult;
use crate::utils::read_line;
use crate::world::World;

// A command line interface for controlling interactions between objects in a game
#[derive(Serialize, Deserialize)]
pub struct Cli {
    player: RefCell<Player>,
    world: RefCell<World>,
}

impl Cli {
    pub fn from_json_file(path: &str) -> Self {
        let world_file = File::open(path).expect("Unable to open world file");
        let mut world_file_reader = BufReader::new(world_file);
        let mut data = String::new();
        world_file_reader
            .read_to_string(&mut data)
            .expect("Unable to read string from world file");

        serde_json::from_str(&data).expect("Error when creating world from file.")
    }

    pub fn from_json_str(json: &str) -> Self {
        serde_json::from_str(json).expect("Error when creating world from file.")
    }

    pub fn start(&self) {
        println!("{}", self.ask("l"));
        loop {
            match self.ask(&self.prompt()) {
                s => {
                    println!("{}", s);
                    if s.contains("You died.") {
                        break;
                    }
                }
            }
        }
    }

    // handle user input
    pub fn ask(&self, input: &str) -> String {
        let filter_out = [
            "a", "an", "at", "go", "my", "of", "that", "the", "through", "to",
        ];

        let mut command = self.parts(input);
        command.retain(|w| !(&filter_out).contains(&w.as_str()));
        let command = self.mod_directions(&command);

        if !command.is_empty() {
            let res = self.parse(&command);
            if res.is_action {
                format!("{}{}", res.command, self.events().unwrap())
            } else {
                res.command
            }
        } else {
            "I do not understand that phrase.".to_string()
        }
    }

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

    fn parts(&self, s: &str) -> Vec<String> {
        s.split_whitespace()
            .map(|x| x.to_lowercase().to_string())
            .collect()
    }

    // modify path directives
    fn mod_directions(&self, words: &[String]) -> Vec<String> {
        let mut modified = Vec::with_capacity(words.len());
        for w in words {
            modified.push(
                match w.as_str() {
                    "north" => "n",
                    "south" => "s",
                    "east" => "e",
                    "west" => "w",
                    "northeast" => "ne",
                    "northwest" => "nw",
                    "southeast" => "se",
                    "southwest" => "sw",
                    "up" => "u",
                    "down" => "d",
                    _ => w,
                }
                .to_string(),
            );
        }
        modified
    }

    fn parse(&self, words: &[String]) -> CmdResult {
        match words[0].as_str() {
            "l" | "look" => CmdResult::new(true, &self.world.borrow().look().unwrap()),
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                CmdResult::new(true, &self.world.borrow_mut().move_room(&words[0]).unwrap())
            }
            "enter" | "go" => {
                if words.len() > 1 {
                    CmdResult::new(true, &self.world.borrow_mut().move_room(&words[1]).unwrap())
                } else {
                    CmdResult::new(false, &format!("Where do you want to {}?", words[0]))
                }
            }
            "i" | "inventory" => CmdResult::new(true, &self.player.borrow().inventory()),
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
                            CmdResult::new(
                                true,
                                &self.player.borrow_mut().take_from(
                                    &words[1..pos].join(" "),
                                    &words[pos + 1..].join(" "),
                                ),
                            )
                        } else {
                            CmdResult::new(
                                true,
                                &self.player.borrow_mut().take(
                                    &words[1..pos].join(" "),
                                    self.world.borrow_mut().give_from(
                                        &words[1..pos].join(" "),
                                        &words[pos + 1..].join(" "),
                                    ),
                                ),
                            )
                        }
                    } else if words[1] == "all" {
                        CmdResult::new(
                            true,
                            &self
                                .player
                                .borrow_mut()
                                .take_all(self.world.borrow_mut().give_all()),
                        )
                    } else if &words[1] == "u" {
                        CmdResult::new(
                            true,
                            &self.player.borrow_mut().take(
                                &words[2..].join(" "),
                                self.world.borrow_mut().give(&words[2..].join(" ")),
                            ),
                        )
                    } else {
                        CmdResult::new(
                            true,
                            &self.player.borrow_mut().take(
                                &words[1..].join(" "),
                                self.world.borrow_mut().give(&words[1..].join(" ")),
                            ),
                        )
                    }
                } else {
                    CmdResult::new(false, &format!("What do you want to {}?", words[0]))
                }
            }
            "drop" | "throw" | "remove" => {
                if words.len() > 1 {
                    CmdResult::new(
                        true,
                        &self
                            .world
                            .borrow_mut()
                            .insert(
                                &words[0],
                                &words[1..].join(" "),
                                self.player.borrow_mut().remove(&words[1..].join(" ")),
                            )
                            .unwrap(),
                    )
                } else {
                    CmdResult::new(
                        false,
                        &format!("What do you want to {} from your inventory?", words[0]),
                    )
                }
            }
            "examine" | "inspect" | "read" => {
                if words.len() > 1 {
                    if let Some(s) = self.player.borrow().inspect(&words[1..].join(" ")) {
                        CmdResult::new(true, &s)
                    } else if let Some(s) = self.world.borrow().inspect(&words[1..].join(" ")) {
                        CmdResult::new(true, &s)
                    } else {
                        CmdResult::new(
                            false,
                            &format!("There is no \"{}\" here.", &words[1..].join(" ")),
                        )
                    }
                } else {
                    CmdResult::new(false, &format!("What do you want to {}?", words[0]))
                }
            }
            "status" | "diagnostic" => CmdResult::new(true, &self.player.borrow().status()),
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
                                CmdResult::new(
                                    true,
                                    &self.player.borrow_mut().put_in(
                                        &words[1..pos].join(" "),
                                        &words[pos + 1..].join(" "),
                                    ),
                                )
                            } else {
                                CmdResult::new(
                                    true,
                                    &self
                                        .world
                                        .borrow_mut()
                                        .insert_into(
                                            &words[1..pos].join(" "),
                                            &words[pos + 1..].join(" "),
                                            self.player
                                                .borrow_mut()
                                                .remove(&words[1..pos].join(" ")),
                                        )
                                        .unwrap(),
                                )
                            }
                        } else if words.len() < 3 {
                            CmdResult::new(false, &format!("What do you want to {}?", words[0]))
                        } else {
                            CmdResult::new(
                                false,
                                &format!(
                                    "What do you want to place in the {}?",
                                    &words[1..].join(" ")
                                ),
                            )
                        }
                    } else {
                        CmdResult::new(
                            false,
                            &format!(
                                "What do you want to {} the {} in?",
                                words[0],
                                &words[1..].join(" ")
                            ),
                        )
                    }
                } else {
                    CmdResult::new(false, &format!("What do you want to {}?", words[0]))
                }
            }
            "attack" | "slay" | "kill" | "hit" => {
                if words.len() > 1 {
                    if let Some(pos) = words.iter().position(|r| r == "with") {
                        let damage = self
                            .player
                            .borrow_mut()
                            .attack_with(&words[pos + 1..].join(" "));
                        CmdResult::new(
                            true,
                            &self
                                .world
                                .borrow_mut()
                                .harm_enemy(
                                    &words[1..pos].join(" "),
                                    &words[pos + 1..].join(" "),
                                    damage,
                                )
                                .unwrap(),
                        )
                    } else if self.player.borrow().main_hand.is_some() {
                        let damage = self.player.borrow_mut().attack();
                        CmdResult::new(
                            true,
                            &self
                                .world
                                .borrow_mut()
                                .harm_enemy(&words[1..].join(" "), "equipped weapon", damage)
                                .unwrap(),
                        )
                    } else {
                        CmdResult::new(
                            false,
                            &format!(
                                "What do you want to {} the {} with?",
                                words[0],
                                &words[1..].join(" ")
                            ),
                        )
                    }
                } else {
                    CmdResult::new(false, &format!("What do you want to {}?", words[0]))
                }
            }
            "rest" | "sleep" | "heal" => {
                if !self.player.borrow().in_combat {
                    CmdResult::new(true, &self.player.borrow_mut().rest())
                } else {
                    CmdResult::new(false, "You cannot rest while in combat.")
                }
            }
            "hold" | "draw" | "equip" => {
                if words.len() > 1 {
                    CmdResult::new(true, &self.player.borrow_mut().equip(&words[1..].join(" ")))
                } else {
                    CmdResult::new(false, &format!("What do you want to {}", words[0]))
                }
            }
            "open" => {
                if words.len() > 1 {
                    CmdResult::new(
                        true,
                        &self
                            .world
                            .borrow_mut()
                            .open_path(&words[1..].join(" "))
                            .unwrap(),
                    )
                } else {
                    CmdResult::new(false, &format!("What do you want to {}", words[0]))
                }
            }
            _ => CmdResult::new(false, &format!("I don't know the word \"{}\".", words[0])),
        }
    }

    // manages actions taken by Enemies in the current room
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
