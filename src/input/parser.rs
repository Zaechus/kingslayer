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
        words: &CmdTokens,
        world: &mut World,
        player: &mut Player,
    ) -> CmdResult {
        if let Some(obj) = words.obj() {
            if let Some(prep) = words.prep() {
                if prep == "with" {
                    if let Some(obj_prep) = words.obj_prep() {
                        world.harm_enemy(player.attack_with(&obj_prep), &obj, &obj_prep)
                    } else {
                        CmdResult::do_what(&format!("{} the {} with?", verb, words.after_verb()))
                    }
                } else {
                    CmdResult::no_comprendo()
                }
            } else {
                let damage = player.attack();

                if let Some(main_hand) = player.main_hand() {
                    world.harm_enemy(damage, obj, &main_hand.name())
                } else {
                    CmdResult::do_what(&format!("{} the {} with?", verb, words.after_verb()))
                }
            }
        } else {
            CmdResult::do_what(verb)
        }
    }

    fn parse_close(
        verb: &str,
        words: &CmdTokens,
        world: &mut World,
        player: &mut Player,
    ) -> CmdResult {
        if let Some(obj) = words.obj() {
            if let Some(res) = player.close(obj) {
                res
            } else {
                world.close(obj)
            }
        } else {
            CmdResult::do_what(verb)
        }
    }

    fn parse_don(verb: &str, words: &CmdTokens, player: &mut Player) -> CmdResult {
        if let Some(obj) = words.obj() {
            player.don_armor(obj)
        } else {
            CmdResult::do_what(verb)
        }
    }

    fn parse_drop(
        verb: &str,
        words: &CmdTokens,
        world: &mut World,
        player: &mut Player,
    ) -> CmdResult {
        if let Some(obj) = words.obj() {
            world.insert(&obj, player.remove(&obj))
        } else {
            CmdResult::do_what(verb)
        }
    }

    fn parse_equip(verb: &str, words: &CmdTokens, player: &mut Player) -> CmdResult {
        if let Some(obj) = words.obj() {
            player.equip(obj)
        } else {
            CmdResult::do_what(verb)
        }
    }

    fn parse_hail(words: &CmdTokens, world: &mut World) -> CmdResult {
        if let Some(obj) = words.obj() {
            world.hail(&obj)
        } else {
            CmdResult::new(Action::Passive, "Who do you want to talk to?".to_owned())
        }
    }

    fn parse_increase(words: &CmdTokens, player: &mut Player) -> CmdResult {
        if let Some(obj) = words.obj() {
            player.increase_ability_score(&obj)
        } else {
            CmdResult::do_what(
                "increase?
                    \r(strength, dexterity, constitution, intellect, wisdom, charisma)",
            )
        }
    }

    fn parse_move(verb: &str, words: &CmdTokens, world: &mut World) -> CmdResult {
        if let Some(obj) = words.obj() {
            world.move_room(&obj)
        } else {
            CmdResult::new(Action::Passive, format!("Where do you want to {}?", verb))
        }
    }

    fn parse_open(
        verb: &str,
        words: &CmdTokens,
        world: &mut World,
        player: &mut Player,
    ) -> CmdResult {
        if let Some(obj) = words.obj() {
            if let Some(res) = player.open(obj) {
                res
            } else {
                world.open(obj)
            }
        } else {
            CmdResult::do_what(verb)
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

    fn parse_take(
        verb: &str,
        words: &CmdTokens,
        world: &mut World,
        player: &mut Player,
    ) -> CmdResult {
        if let Some(obj) = words.obj() {
            if let Some(prep) = words.prep() {
                if prep == "from" || prep == "out" || prep == "in" {
                    if let Some(obj_prep) = words.obj_prep() {
                        if player.has(obj_prep) {
                            player.take_from_self(obj, obj_prep)
                        } else {
                            player.take_item_from(world.give_from(obj, obj_prep))
                        }
                    } else {
                        CmdResult::no_comprendo()
                    }
                } else {
                    CmdResult::do_what(&format!("{} the {} with?", verb, words.after_verb()))
                }
            } else if obj.starts_with("all") || obj.len() >= 4 && obj.starts_with("all ") {
                player.take_all(world.give_all())
            } else if obj.starts_with("u ") {
                player.take(&obj[2..], world.give(&obj[2..]))
            } else {
                player.take(obj, world.give(obj))
            }
        } else {
            CmdResult::do_what(verb)
        }
    }

    fn parse_x(verb: &str, words: &CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        if let Some(obj) = words.obj() {
            if let Some(s) = player.inspect(obj) {
                s
            } else if let Some(s) = world.inspect(obj) {
                s
            } else {
                CmdResult::no_item_here(obj)
            }
        } else {
            CmdResult::do_what(verb)
        }
    }

    pub fn parse(words: CmdTokens, world: &mut World, player: &mut Player) -> CmdResult {
        if let Some(verb) = words.short_verb() {
            match verb {
                "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
                    world.move_room(verb)
                }
                "enter" | "go" | "move" => Parser::parse_move(verb, &words, world),
                "c" | "stat" | "stats" => player.info(),
                "i" | "invent" => player.print_inventory(),
                "l" | "look" => world.look(),
                "attack" | "cut" | "hit" | "kill" | "slay" => {
                    Parser::parse_attack(verb, &words, world, player)
                }
                "heal" | "rest" | "sleep" => player.rest(),
                "hail" | "talk" | "hi" | "hello" | "greet" => Parser::parse_hail(&words, world),
                "close" => Parser::parse_close(verb, &words, world, player),
                "don" => Parser::parse_don(verb, &words, player),
                "draw" | "equip" | "hold" | "use" => Parser::parse_equip(verb, &words, player),
                "drop" | "remove" | "throw" => Parser::parse_drop(verb, &words, world, player),
                "examin" | "inspec" | "read" | "x" => Parser::parse_x(verb, &words, world, player),
                "get" | "pick" | "take" => Parser::parse_take(verb, &words, world, player),
                "increa" => Parser::parse_increase(&words, player),
                "open" => Parser::parse_open(verb, &words, world, player),
                "insert" | "place" | "put" => Parser::parse_put(&words, verb, world, player),
                "save" => Cli::save(world),
                "wait" | "z" => Player::wait(),
                "help" => Cli::help(),
                _ => CmdResult::new(
                    Action::Passive,
                    format!("I do not know the word \"{}\"", verb),
                ),
            }
        } else {
            CmdResult::no_comprendo()
        }
    }
}
