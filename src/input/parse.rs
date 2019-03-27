use crate::{cli::Cli, player::Player, types::CmdResult, world::World};

fn do_what(word: &str) -> CmdResult {
    CmdResult::new(false, format!("What do you want to {}?", word))
}

pub fn parse(words: &[String], world: &mut World, player: &mut Player) -> CmdResult {
    match if words[0].len() >= 6 {
        &words[0][0..6]
    } else {
        &words[0]
    } {
        "l" | "look" => world.look().unwrap(),
        "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw" | "u" | "d" => {
            world.move_room(&words[0]).unwrap()
        }
        "enter" | "go" | "move" => {
            if words.len() > 1 {
                world.move_room(&words[1]).unwrap()
            } else {
                CmdResult::new(false, format!("Where do you want to {}?", words[0]))
            }
        }
        "i" | "invent" => player.inventory(),
        "take" | "get" | "pick" => {
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
                do_what(&words[0])
            }
        }
        "drop" | "throw" | "remove" | "give" => {
            if words.len() > 1 {
                world
                    .insert(&words[1..].join(" "), player.remove(&words[1..].join(" ")))
                    .unwrap()
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
        "status" | "diagno" => player.status(),
        "put" | "place" => {
            if words.len() > 1 {
                if let Some(pos) = words.iter().position(|r| r == "in" || r == "inside") {
                    if pos != 1 {
                        if player.has(&words[pos + 1..].join(" ")) {
                            player.put_in(&words[1..pos].join(" "), &words[pos + 1..].join(" "))
                        } else {
                            world
                                .insert_into(
                                    &words[1..pos].join(" "),
                                    &words[pos + 1..].join(" "),
                                    player.remove(&words[1..pos].join(" ")),
                                )
                                .unwrap()
                        }
                    } else if words.len() < 3 {
                        do_what(&words[0])
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
                do_what(&words[0])
            }
        }
        "attack" | "slay" | "kill" | "hit" | "cut" => {
            if words.len() > 1 {
                if let Some(pos) = words.iter().position(|r| r == "with") {
                    let damage = player.attack_with(&words[pos + 1..].join(" "));

                    world
                        .harm_enemy(
                            damage,
                            &words[1..pos].join(" "),
                            &words[pos + 1..].join(" "),
                        )
                        .unwrap()
                } else {
                    let damage = player.attack();

                    if let Some(main_hand) = player.main_hand() {
                        world
                            .harm_enemy(damage, &words[1..].join(" "), &main_hand.name())
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
                }
            } else {
                do_what(&words[0])
            }
        }
        "rest" | "sleep" | "heal" => player.rest(),
        "hold" | "draw" | "equip" | "use" => {
            if words.len() > 1 {
                player.equip(&words[1..].join(" "))
            } else {
                do_what(&words[0])
            }
        }
        "open" => {
            if words.len() > 1 {
                world.open_path(&words[1..].join(" ")).unwrap()
            } else {
                do_what(&words[0])
            }
        }
        "close" => {
            if words.len() > 1 {
                world.close_path(&words[1..].join(" ")).unwrap()
            } else {
                do_what(&words[0])
            }
        }
        "increa" => {
            if words.len() > 1 {
                player.increase_ability_score(&words[1])
            } else {
                CmdResult::new(
                    false,
                    "What do you want to increase?
                    \r(strength, dexterity, constitution, intellect, wisdom, charisma)"
                        .to_string(),
                )
            }
        }
        "z" | "wait" => Player::wait(),
        "help" => Cli::help(),
        _ => CmdResult::new(false, format!("I don't know the word \"{}\".", words[0])),
    }
}
