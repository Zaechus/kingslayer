use crate::{player::Player, types::CmdResult, world::World};

fn do_what(word: &str) -> String {
    format!("What do you want to {}?", word)
}

pub fn parse(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
    match if words[0].len() >= 6 {
        &words[0][0..6]
    } else {
        &words[0]
    } {
        "l" | "look" => CmdResult::new(true, world.look().unwrap()),
        "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
            CmdResult::new(true, world.move_room(&words[0]).unwrap())
        }
        "enter" | "go" => {
            if words.len() > 1 {
                CmdResult::new(true, world.move_room(&words[1]).unwrap())
            } else {
                CmdResult::new(false, format!("Where do you want to {}?", words[0]))
            }
        }
        "i" | "invent" => CmdResult::new(true, player.inventory()),
        "take" | "get" | "pick" => {
            if words.len() > 1 {
                if let Some(pos) = words
                    .iter()
                    .position(|r| r == "from" || r == "out" || r == "in")
                {
                    if player.inventory().contains(&words[pos + 1..].join(" ")) {
                        CmdResult::new(
                            true,
                            player.take_from(&words[1..pos].join(" "), &words[pos + 1..].join(" ")),
                        )
                    } else {
                        CmdResult::new(
                            true,
                            player.take(
                                &words[1..pos].join(" "),
                                world.give_from(
                                    &words[1..pos].join(" "),
                                    &words[pos + 1..].join(" "),
                                ),
                            ),
                        )
                    }
                } else if words[1] == "all" {
                    CmdResult::new(true, player.take_all(world.give_all()))
                } else if &words[1] == "u" {
                    CmdResult::new(
                        true,
                        player.take(&words[2..].join(" "), world.give(&words[2..].join(" "))),
                    )
                } else {
                    CmdResult::new(
                        true,
                        player.take(&words[1..].join(" "), world.give(&words[1..].join(" "))),
                    )
                }
            } else {
                CmdResult::new(false, do_what(&words[0]))
            }
        }
        "drop" | "throw" | "remove" => {
            if words.len() > 1 {
                CmdResult::new(
                    true,
                    world
                        .insert(
                            &words[0],
                            &words[1..].join(" "),
                            player.remove(&words[1..].join(" ")),
                        )
                        .unwrap(),
                )
            } else {
                CmdResult::new(
                    false,
                    format!("What do you want to {} from your inventory?", words[0]),
                )
            }
        }
        "x" | "examin" | "inspec" | "read" => {
            if words.len() > 1 {
                if let Some(s) = player.inspect(&words[1..].join(" ")) {
                    CmdResult::new(true, s)
                } else if let Some(s) = world.inspect(&words[1..].join(" ")) {
                    CmdResult::new(true, s)
                } else {
                    CmdResult::new(
                        false,
                        format!("There is no \"{}\" here.", &words[1..].join(" ")),
                    )
                }
            } else {
                CmdResult::new(false, do_what(&words[0]))
            }
        }
        "status" | "diagno" => CmdResult::new(true, player.status()),
        "put" | "place" => {
            if words.len() > 1 {
                if let Some(pos) = words.iter().position(|r| r == "in" || r == "inside") {
                    if pos != 1 {
                        if player.inventory().contains(&words[pos + 1..].join(" ")) {
                            CmdResult::new(
                                true,
                                player
                                    .put_in(&words[1..pos].join(" "), &words[pos + 1..].join(" ")),
                            )
                        } else {
                            CmdResult::new(
                                true,
                                world
                                    .insert_into(
                                        &words[1..pos].join(" "),
                                        &words[pos + 1..].join(" "),
                                        player.remove(&words[1..pos].join(" ")),
                                    )
                                    .unwrap(),
                            )
                        }
                    } else if words.len() < 3 {
                        CmdResult::new(false, do_what(&words[0]))
                    } else {
                        CmdResult::new(
                            false,
                            format!(
                                "What do you want to place in the {}?",
                                &words[1..].join(" ")
                            ),
                        )
                    }
                } else {
                    CmdResult::new(
                        false,
                        format!(
                            "What do you want to {} the {} in?",
                            words[0],
                            &words[1..].join(" ")
                        ),
                    )
                }
            } else {
                CmdResult::new(false, do_what(&words[0]))
            }
        }
        "attack" | "slay" | "kill" | "hit" | "cut" => {
            if words.len() > 1 {
                if let Some(pos) = words.iter().position(|r| r == "with") {
                    let damage = player.attack_with(&words[pos + 1..].join(" "));

                    world
                        .harm_enemy(
                            &words[1..pos].join(" "),
                            &words[pos + 1..].join(" "),
                            damage,
                        )
                        .unwrap()
                } else if player.main_hand().is_some() {
                    let damage = player.attack();
                    world
                        .harm_enemy(&words[1..].join(" "), "equipped weapon", damage)
                        .unwrap()
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
            } else {
                CmdResult::new(false, do_what(&words[0]))
            }
        }
        "rest" | "sleep" | "heal" => player.rest(),
        "hold" | "draw" | "equip" => {
            if words.len() > 1 {
                CmdResult::new(true, player.equip(&words[1..].join(" ")))
            } else {
                CmdResult::new(false, do_what(&words[0]))
            }
        }
        "open" => {
            if words.len() > 1 {
                CmdResult::new(true, world.open_path(&words[1..].join(" ")).unwrap())
            } else {
                CmdResult::new(false, do_what(&words[0]))
            }
        }
        "close" => {
            if words.len() > 1 {
                CmdResult::new(true, world.close_path(&words[1..].join(" ")).unwrap())
            } else {
                CmdResult::new(false, do_what(&words[0]))
            }
        }
        "z" | "wait" => Player::wait(),
        _ => CmdResult::new(false, format!("I don't know the word \"{}\".", words[0])),
    }
}
