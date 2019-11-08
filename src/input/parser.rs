use serde::{Deserialize, Serialize};

use crate::{
    cli::Cli,
    entity::Entity,
    player::Player,
    types::{Action, CmdResult, CmdTokens},
    world::World,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parser;

impl Parser {
    fn parse_attack(
        verb: &str,
        after_verb: &str,
        obj: &str,
        world: &mut World,
        player: &mut Player,
    ) -> CmdResult {
        let damage = player.attack();

        if let Some(main_hand) = player.main_hand() {
            world.harm_enemy(damage, obj, &main_hand.name())
        } else {
            CmdResult::do_what(&format!("{} the {} with?", verb, after_verb))
        }
    }

    fn parse_close(obj: &str, world: &mut World, player: &mut Player) -> CmdResult {
        if let Some(res) = player.close(obj) {
            res
        } else {
            world.close(obj)
        }
    }

    fn parse_open(obj: &str, world: &mut World, player: &mut Player) -> CmdResult {
        if let Some(res) = player.open(obj) {
            res
        } else {
            world.open(obj)
        }
    }

    fn parse_put(
        words: &CmdTokens,
        verb: &str,
        world: &mut World,
        player: &mut Player,
    ) -> CmdResult {
        if let Some(prep) = words.prep() {
            match prep.as_str() {
                "in" | "inside" => {
                    if let Some(obj) = words.obj() {
                        if let Some(obj_prep) = words.obj_prep() {
                            if player.has(&obj_prep) {
                                player.insert_into(&obj, &obj_prep)
                            } else {
                                let (res, rejected_item) =
                                    world.insert_into(&obj, &obj_prep, player.remove(&obj));
                                if let Some(item) = rejected_item {
                                    player.take_back(item);
                                    res
                                } else {
                                    res
                                }
                            }
                        } else {
                            CmdResult::do_what(&format!("place in the {}?", words.after_verb()))
                        }
                    } else {
                        CmdResult::do_what(verb)
                    }
                }
                "on" => {
                    if let Some(obj_prep) = words.obj_prep() {
                        player.don_armor(&obj_prep)
                    } else {
                        CmdResult::do_what(&format!("{} on", verb))
                    }
                }
                _ => CmdResult::no_comprendo(),
            }
        } else {
            CmdResult::do_what(&format!("{} the {} in", verb, words.after_verb()))
        }
    }

    fn parse_take(obj: &str, world: &mut World, player: &mut Player) -> CmdResult {
        if obj.starts_with("all") || obj.len() >= 4 && obj.starts_with("all ") {
            player.take_all(world.give_all())
        } else if obj.starts_with("u ") {
            player.take(&obj[2..], world.give(&obj[2..]))
        } else {
            player.take(obj, world.give(obj))
        }
    }

    fn parse_take_from(
        obj: &str,
        prep: &str,
        obj_prep: &str,
        world: &mut World,
        player: &mut Player,
    ) -> CmdResult {
        if prep == "from" || prep == "out" || prep == "in" {
            if player.has(obj_prep) {
                player.take_from_self(obj, obj_prep)
            } else {
                player.take_item_from(world.give_from(obj, obj_prep))
            }
        } else {
            CmdResult::no_comprendo()
        }
    }

    fn parse_x(obj: &str, world: &mut World, player: &mut Player) -> CmdResult {
        if let Some(s) = player.inspect(obj) {
            s
        } else if let Some(s) = world.inspect(obj) {
            s
        } else {
            CmdResult::no_item_here(obj)
        }
    }

    pub fn parse(words: CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        if let Some(verb) = words.short_verb() {
            match verb {
                "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                    world.move_room(verb)
                }
                "c" | "stat" | "stats" => player.info(),
                "heal" | "rest" | "sleep" => player.rest(),
                "help" => Cli::help(),
                "i" | "invent" => player.print_inventory(),
                "l" | "look" => world.look(),
                "enter" | "go" | "move" => {
                    if let Some(obj) = words.obj() {
                        world.move_room(&obj)
                    } else {
                        CmdResult::new(Action::Passive, format!("Where do you want to {}?", verb))
                    }
                }
                "hail" | "talk" | "hi" | "hello" | "greet" => {
                    if let Some(obj) = words.obj() {
                        world.hail(&obj)
                    } else {
                        CmdResult::new(Action::Passive, "Who do you want to talk to?".to_owned())
                    }
                }
                "increa" => {
                    if let Some(obj) = words.obj() {
                        player.increase_ability_score(&obj)
                    } else {
                        CmdResult::do_what(
                            "increase?
                    \r(strength, dexterity, constitution, intellect, wisdom, charisma)",
                        )
                    }
                }
                "wait" | "z" => Player::wait(),
                "insert" | "place" | "put" => Parser::parse_put(&words, verb, world, player),
                _ => {
                    if let Some(obj) = words.obj() {
                        match verb {
                            "attack" | "cut" | "hit" | "kill" | "slay" => {
                                if let Some(prep) = words.prep() {
                                    if prep == "with" {
                                        world.harm_enemy(
                                            player.attack_with(&words.obj_prep().unwrap()),
                                            &obj,
                                            &words.obj_prep().unwrap(),
                                        )
                                    } else {
                                        CmdResult::no_comprendo()
                                    }
                                } else {
                                    Parser::parse_attack(
                                        verb,
                                        &words.after_verb(),
                                        &obj,
                                        world,
                                        player,
                                    )
                                }
                            }
                            "close" => Parser::parse_close(&obj, world, player),
                            "don" => player.don_armor(&obj),
                            "draw" | "equip" | "hold" | "use" => player.equip(&obj),
                            "drop" | "remove" | "throw" => world.insert(&obj, player.remove(&obj)),
                            "examin" | "inspec" | "read" | "x" => {
                                Parser::parse_x(&obj, world, player)
                            }
                            "get" | "pick" | "take" => {
                                if let Some(prep) = words.prep() {
                                    Parser::parse_take_from(
                                        &obj,
                                        &prep,
                                        &words.obj_prep().unwrap(),
                                        world,
                                        player,
                                    )
                                } else {
                                    Parser::parse_take(&obj, world, player)
                                }
                            }
                            "open" => Parser::parse_open(&obj, world, player),
                            "save" => Cli::save(world),
                            _ => CmdResult::new(
                                Action::Passive,
                                format!("I do not know the word \"{}\".", verb,),
                            ),
                        }
                    } else {
                        CmdResult::do_what(verb)
                    }
                }
            }
        } else {
            CmdResult::no_comprendo()
        }
    }
}
