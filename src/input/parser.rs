use serde_derive::{Deserialize, Serialize};

use crate::{
    cli::Cli,
    player::Player,
    types::{CmdResult, CmdTokens},
    util::do_what,
    world::World,
};

#[derive(Serialize, Deserialize)]
pub struct Parser;

impl Parser {
    fn parse_attack(words: CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        if words.num_words() > 1 {
            if words.prep() == "with" {
                world.harm_enemy(
                    player.attack_with(words.obj_prep()),
                    words.obj(),
                    words.obj_prep(),
                )
            } else {
                let damage = player.attack();

                if let Some(main_hand) = player.main_hand() {
                    world.harm_enemy(damage, &words.obj(), &main_hand.name())
                } else {
                    do_what(&format!(
                        "{} the {} with?",
                        words.verb(),
                        words.after_verb()
                    ))
                }
            }
        } else {
            do_what(words.verb())
        }
    }

    fn parse_close(words: CmdTokens, player: &mut Player, world: &mut World) -> CmdResult {
        if words.num_words() > 1 {
            if player.has(&words.obj()) {
                player.close(&words.obj())
            } else {
                world.close(&words.obj())
            }
        } else {
            do_what(words.verb())
        }
    }

    fn parse_don(words: CmdTokens, player: &mut Player) -> CmdResult {
        if words.num_words() > 1 {
            player.don_armor(&words.obj())
        } else {
            do_what(words.verb())
        }
    }

    fn parse_drop(words: CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        if words.num_words() > 1 {
            world.insert(&words.obj(), player.remove(&words.obj()))
        } else {
            do_what(&format!("{} from your inventory?", words.verb()))
        }
    }

    fn parse_equip(words: CmdTokens, player: &mut Player) -> CmdResult {
        if words.num_words() > 1 {
            player.equip(&words.obj())
        } else {
            do_what(words.verb())
        }
    }

    fn parse_go(words: CmdTokens, world: &mut World) -> CmdResult {
        if words.num_words() > 1 {
            world.move_room(&words.obj())
        } else {
            CmdResult::new(false, format!("Where do you want to {}?", words.verb()))
        }
    }

    fn parse_increase(words: CmdTokens, player: &mut Player) -> CmdResult {
        if words.num_words() > 1 && words.obj().len() >= 3 {
            player.increase_ability_mod(&words.obj())
        } else {
            do_what(
                "increase?
                    \r(strength, dexterity, constitution, intellect, wisdom, charisma)",
            )
        }
    }

    fn parse_hail(words: CmdTokens, world: &mut World) -> CmdResult {
        if words.num_words() > 1 {
            world.hail(&words.obj())
        } else {
            CmdResult::new(false, "Who do you want to talk to?".to_owned())
        }
    }

    fn parse_open(words: CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        let obj = words.obj();
        if words.num_words() > 1 {
            if player.has(&obj) {
                player.open(&obj)
            } else {
                world.open(&obj)
            }
        } else {
            do_what(words.verb())
        }
    }

    fn parse_put(words: CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        if words.num_words() > 1 {
            if words.prep() == "in" || words.prep() == "inside" {
                if !words.obj().is_empty() {
                    if player.has(&words.obj_prep()) {
                        player.insert_into(&words.obj(), &words.obj_prep())
                    } else {
                        world.insert_into(player, &words.obj(), &words.obj_prep())
                    }
                } else if words.num_words() < 3 {
                    do_what(words.verb())
                } else {
                    do_what(&format!("place in the {}?", words.after_verb()))
                }
            } else if words.prep() == "on" {
                if words.num_words() > 2 {
                    player.don_armor(&words.obj_prep())
                } else {
                    do_what(&format!("{} on", words.verb()))
                }
            } else {
                do_what(&format!("{} the {} in", words.verb(), words.after_verb()))
            }
        } else {
            do_what(words.verb())
        }
    }

    fn parse_stats(player: &mut Player) -> CmdResult {
        player.stats()
    }

    fn parse_take(words: CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        if words.num_words() > 1 {
            if words.prep() == "from" || words.prep() == "out" || words.prep() == "in" {
                if player.has(&words.obj_prep()) {
                    player.take_from(&words.obj(), &words.obj_prep())
                } else {
                    world.give_from(player, &words.obj(), &words.obj_prep())
                }
            } else if words.obj().len() >= 3 && words.obj().get(0..3).unwrap() == "all" {
                player.take_all(world.give_all())
            } else if words.after_verb_vec()[0] == "u" {
                player.take(
                    &words.after_verb()[2..],
                    world.give(&words.after_verb()[2..]),
                )
            } else {
                player.take(&words.obj(), world.give(&words.obj()))
            }
        } else {
            do_what(words.verb())
        }
    }

    fn parse_x(words: CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        if words.num_words() > 1 {
            if let Some(s) = player.inspect(&words.obj()) {
                s
            } else if let Some(s) = world.inspect(&words.obj()) {
                s
            } else {
                CmdResult::new(false, format!("There is no \"{}\" here.", words.obj()))
            }
        } else {
            do_what(words.verb())
        }
    }

    pub fn parse(words: CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        match if words.verb().len() >= 6 {
            &words.verb()[0..6]
        } else {
            words.verb()
        } {
            "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                world.move_room(words.verb())
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
            "hail" | "talk" | "hi" | "hello" | "greet" => Parser::parse_hail(words, world),
            "heal" | "rest" | "sleep" => player.rest(),
            "help" => Cli::help(),
            "i" | "invent" => player.print_inventory(),
            "increa" => Parser::parse_increase(words, player),
            "l" | "look" => world.look(),
            "open" => Parser::parse_open(words, world, player),
            "insert" | "place" | "put" => Parser::parse_put(words, world, player),
            "stats" => Parser::parse_stats(player),
            "wait" | "z" => Player::wait(),
            _ => CmdResult::new(
                false,
                format!("I do not know the word \"{}\".", words.verb()),
            ),
        }
    }
}
