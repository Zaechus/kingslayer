use serde_derive::{Deserialize, Serialize};

use crate::{cli::Cli, player::Player, response::do_what, types::CmdResult, world::World};

#[derive(Serialize, Deserialize)]
pub struct Parser;

impl Parser {
    fn parse_attack(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            if let Some(pos) = words.iter().position(|r| r == "with") {
                world.harm_enemy(
                    player.attack_with(&words[pos + 1..].join(" ")),
                    &words[1..pos].join(" "),
                    &words[pos + 1..].join(" "),
                )
            } else {
                let damage = player.attack();

                if let Some(main_hand) = player.main_hand() {
                    world.harm_enemy(damage, &words[1..].join(" "), &main_hand.name())
                } else {
                    do_what(&format!("{} the {} with?", words[0], &words[1..].join(" ")))
                }
            }
        } else {
            do_what(&words[0])
        }
    }

    fn parse_close(words: &[String], player: &mut Player, world: &mut World) -> CmdResult {
        let obj = &words[1..].join(" ");
        if words.len() > 1 {
            if player.has(obj) {
                player.close(obj)
            } else {
                world.close(obj)
            }
        } else {
            do_what(&words[0])
        }
    }

    fn parse_don(words: &[String], player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            player.don_armor(&words[1..].join(" "))
        } else {
            do_what(&words[0])
        }
    }

    fn parse_drop(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            world.insert(&words[1..].join(" "), player.remove(&words[1..].join(" ")))
        } else {
            do_what(&format!("{} from your inventory?", words[0]))
        }
    }

    fn parse_equip(words: &[String], player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            player.equip(&words[1..].join(" "))
        } else {
            do_what(&words[0])
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
            do_what(
                "increase?
                    \r(strength, dexterity, constitution, intellect, wisdom, charisma)",
            )
        }
    }

    fn parse_open(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        let obj = &words[1..].join(" ");
        if words.len() > 1 {
            if player.has(obj) {
                player.open(obj)
            } else {
                world.open(obj)
            }
        } else {
            do_what(&words[0])
        }
    }

    fn parse_put(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
        if words.len() > 1 {
            if let Some(pos) = words.iter().position(|r| r == "in" || r == "inside") {
                if pos != 1 {
                    if player.has(&words[pos + 1..].join(" ")) {
                        player.insert_into(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                    } else {
                        world.insert_into(
                            player,
                            &words[1..pos].join(" "),
                            &words[pos + 1..].join(" "),
                        )
                    }
                } else if words.len() < 3 {
                    do_what(&words[0])
                } else {
                    do_what(&format!("place in the {}?", &words[1..].join(" ")))
                }
            } else if &words[1] == "on" {
                if words.len() > 2 {
                    player.don_armor(&words[1..].join(" "))
                } else {
                    do_what(&format!("{} on", &words[0]))
                }
            } else {
                do_what(&format!("{} the {} in", words[0], &words[1..].join(" ")))
            }
        } else {
            do_what(&words[0])
        }
    }

    fn parse_stats(player: &mut Player) -> CmdResult {
        player.stats()
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
                    world.give_from(
                        player,
                        &words[1..pos].join(" "),
                        &words[pos + 1..].join(" "),
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
            do_what(&words[0])
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
            do_what(&words[0])
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
            "close" => Parser::parse_close(words, player, world),
            "diagno" | "status" => player.status(),
            "don" => Parser::parse_don(words, player),
            "draw" | "equip" | "hold" | "use" => Parser::parse_equip(words, player),
            "drop" | "remove" | "throw" => Parser::parse_drop(words, world, player),
            "enter" | "go" | "move" => Parser::parse_go(words, world),
            "examin" | "inspec" | "read" | "x" => Parser::parse_x(words, world, player),
            "get" | "pick" | "take" => Parser::parse_take(words, world, player),
            "heal" | "rest" | "sleep" => player.rest(),
            "help" => Cli::help(),
            "i" | "invent" => player.print_inventory(),
            "increa" => Parser::parse_increase(words, player),
            "l" | "look" => world.look(),
            "open" => Parser::parse_open(words, world, player),
            "insert" | "place" | "put" => Parser::parse_put(words, world, player),
            "stats" => Parser::parse_stats(player),
            "wait" | "z" => Player::wait(),
            _ => CmdResult::new(false, format!("I do not know the word \"{}\".", &words[0])),
        }
    }
}
