use serde_derive::{Deserialize, Serialize};

use crate::{cli::Cli, player::Player, types::CmdResult, world::World};

#[derive(Serialize, Deserialize)]
pub struct Parser;

impl Parser {
    fn do_what(word: &str) -> CmdResult {
        CmdResult::new(false, format!("What do you want to {}?", word))
    }

    fn parse_attack(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            if let Some(pos) = words.iter().position(|r| r == "with") {
                let damage = player.attack_with(&words[pos + 1..].join(" "));

                world.harm_enemy(
                    damage,
                    &words[1..pos].join(" "),
                    &words[pos + 1..].join(" "),
                )
            } else {
                let damage = player.attack();

                if let Some(main_hand) = player.main_hand() {
                    world.harm_enemy(damage, &words[1..].join(" "), &main_hand.name())
                } else {
                    CmdResult::new(
                        false,
                        format!(
                            "What do you want to {} the {} with?",
                            words[0],
                            &words[1..].join(" ")
                        ),
                    )
                }
            }
        } else {
            Parser::do_what(&words[0])
        }
    }

    fn parse_close(words: &[String], world: &mut World) -> CmdResult {
        if words.len() > 1 {
            world.close_path(&words[1..].join(" "))
        } else {
            Parser::do_what(&words[0])
        }
    }

    fn parse_don(words: &[String], player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            player.don_armor(&words[1..].join(" "))
        } else {
            Parser::do_what(&words[0])
        }
    }

    fn parse_drop(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            world.insert(&words[1..].join(" "), player.remove(&words[1..].join(" ")))
        } else {
            CmdResult::new(
                false,
                format!("What do you want to {} from your inventory?", words[0]),
            )
        }
    }

    fn parse_equip(words: &[String], player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            player.equip(&words[1..].join(" "))
        } else {
            Parser::do_what(&words[0])
        }
    }

    fn parse_go(words: &[String], world: &mut World) -> CmdResult {
        if words.len() > 1 {
            world.move_room(&words[1])
        } else {
            CmdResult::new(false, format!("Where do you want to {}?", words[0]))
        }
    }

    fn parse_increase(words: &[String], player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            player.increase_ability_mod(&words[1])
        } else {
            CmdResult::new(
                false,
                "What do you want to increase?
                    \r(strength, dexterity, constitution, intellect, wisdom, charisma)"
                    .to_string(),
            )
        }
    }

    fn parse_open(words: &[String], world: &mut World) -> CmdResult {
        let obj = &words[1..].join(" ");
        if words.len() > 1 {
            if world.get_curr_room().has_path(obj) || world.get_curr_room().has_item(obj) {
                world.open(obj)
            } else {
                CmdResult::new(false, "TODO: Player open".to_string())
            }
        } else {
            Parser::do_what(&words[0])
        }
    }

    fn parse_put(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            if let Some(pos) = words.iter().position(|r| r == "in" || r == "inside") {
                if pos != 1 {
                    if player.has(&words[pos + 1..].join(" ")) {
                        player.put_in(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                    } else {
                        world.insert_into(
                            &words[1..pos].join(" "),
                            &words[pos + 1..].join(" "),
                            player.remove(&words[1..pos].join(" ")),
                        )
                    }
                } else if words.len() < 3 {
                    Parser::do_what(&words[0])
                } else {
                    CmdResult::new(
                        false,
                        format!(
                            "What do you want to place in the {}?",
                            &words[1..].join(" ")
                        ),
                    )
                }
            } else if &words[1] == "on" {
                if words.len() > 2 {
                    player.don_armor(&words[1..].join(" "))
                } else {
                    Parser::do_what(&format!("{} on", &words[0]))
                }
            } else {
                Parser::do_what(&format!("{} the {} in", words[0], &words[1..].join(" ")))
            }
        } else {
            Parser::do_what(&words[0])
        }
    }

    fn parse_take(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            if let Some(pos) = words
                .iter()
                .position(|r| r == "from" || r == "out" || r == "in")
            {
                if player.has(&words[pos + 1..].join(" ")) {
                    player.take_from(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                } else {
                    player.take(
                        &words[1..pos].join(" "),
                        world.give_from(&words[1..pos].join(" "), &words[pos + 1..].join(" ")),
                    )
                }
            } else if words[1] == "all" {
                player.take_all(world.give_all())
            } else if &words[1] == "u" {
                player.take(&words[2..].join(" "), world.give(&words[2..].join(" ")))
            } else {
                player.take(&words[1..].join(" "), world.give(&words[1..].join(" ")))
            }
        } else {
            Parser::do_what(&words[0])
        }
    }

    fn parse_x(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            if let Some(s) = player.inspect(&words[1..].join(" ")) {
                s
            } else if let Some(s) = world.inspect(&words[1..].join(" ")) {
                s
            } else {
                CmdResult::new(
                    false,
                    format!("There is no \"{}\" here.", &words[1..].join(" ")),
                )
            }
        } else {
            Parser::do_what(&words[0])
        }
    }

    pub fn parse(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        match if words[0].len() >= 6 {
            &words[0][0..6]
        } else {
            &words[0]
        } {
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                world.move_room(&words[0])
            }
            "attack" | "cut" | "hit" | "kill" | "slay" => {
                Parser::parse_attack(words, world, player)
            }
            "close" => Parser::parse_close(words, world),
            "diagno" | "status" => player.status(),
            "don" => Parser::parse_don(words, player),
            "draw" | "equip" | "hold" | "use" => Parser::parse_equip(words, player),
            "drop" | "remove" | "throw" => Parser::parse_drop(words, world, player),
            "enter" | "go" | "move" => Parser::parse_go(words, world),
            "examin" | "inspec" | "read" | "x" => Parser::parse_x(words, world, player),
            "get" | "pick" | "take" => Parser::parse_take(words, world, player),
            "heal" | "rest" | "sleep" => player.rest(),
            "help" => Cli::help(),
            "i" | "invent" => player.inventory(),
            "increa" => Parser::parse_increase(words, player),
            "l" | "look" => world.look(),
            "open" => Parser::parse_open(words, world),
            "place" | "put" => Parser::parse_put(words, world, player),
            "wait" | "z" => Player::wait(),
            _ => CmdResult::new(false, format!("I do not know the word \"{}\".", &words[0])),
        }
    }
}
