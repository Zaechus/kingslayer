use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

use serde_derive::{Deserialize, Serialize};

use crate::{
    input::{parse, Lexer},
    player::Player,
    types::{CmdResult, ItemMap, WorldError},
    utils::read_line,
    world::World,
};

// A command line interface for controlling interactions between objects in a game
#[derive(Serialize, Deserialize)]
pub struct Cli {
    lexer: Lexer,
    player: RefCell<Player>,
    world: RefCell<World>,
}

impl Cli {
    pub fn from_json_file(path: &str) -> Self {
        Self {
            lexer: Lexer::new(),
            player: RefCell::new(Player::new()),
            world: Cli::get_world_json(path),
        }
    }

    pub fn from_json_str(json: &str) -> Self {
        Self {
            lexer: Lexer::new(),
            player: RefCell::new(Player::new()),
            world: Cli::get_world_str(json),
        }
    }

    fn get_world_json(path: &str) -> RefCell<World> {
        let world_file = File::open(path).expect("Unable to open world file");
        let mut world_file_reader = BufReader::new(world_file);
        let mut data = String::new();
        world_file_reader
            .read_to_string(&mut data)
            .expect("Unable to read string from world file");

        serde_json::from_str(&data).expect("Error when creating world from file.")
    }

    fn get_world_str(json: &str) -> RefCell<World> {
        serde_json::from_str(json).expect("Error when creating world from file.")
    }

    pub fn prompt() -> String {
        loop {
            print!("\n> ");
            io::stdout()
                .flush()
                .expect("There was a problem flushing stdout");
            let input = read_line();
            if !input.is_empty() {
                return input;
            } else {
                println!("Excuse me?");
            }
        }
    }

    pub fn help() -> CmdResult {
        let mut res = String::with_capacity(666);
        res.push_str("Some available commands:\n");
        res.push_str("\tgo, enter <direction>\tmove in through a listed entrance\n");
        res.push_str("\t\tshort directions: n, s, e, w, ne, nw, se, sw, u, d\n");
        res.push_str("\t\tlong directions:\n");
        res.push_str("\t\t   north, south, east, west,\n");
        res.push_str("\t\t   northeast, northwest, southeast, southwest,\n");
        res.push_str("\t\t   up, down, (other listed entrance)\n");
        res.push_str("\ttake\t\tput an item from the room into your inventory\n");
        res.push_str("\tdrop\t\tdrop an item from your inventory into the room\n");
        res.push_str("\tl, look\t\tlook around the room\n");
        res.push_str("\ti, inventory\tprint the contents of your inventory\n");
        res.push_str("\tx, examine\tshow additional information about an item\n");
        res.push_str("\tequip\t\tuse an item from your inventory as your default weapon\n");
        res.push_str("\tkill\t\tattack an enemy with your main hand or a chosen weapon\n");
        res.push_str("\topen | close\topen/close a container or pathway\n");
        res.push_str("\trest\t\treplenish some HP\n");
        res.push_str("\tstatus\t\tdisplay your HP");
        CmdResult::new(false, res)
    }

    pub fn start(&self) {
        println!("Type \"help\" to learn come commands.\n\n{}", self.ask("l"));
        loop {
            match self.ask(&Cli::prompt()) {
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
        let command = self.lexer.lex(input);

        if !command.is_empty() {
            let res = parse(
                &command,
                &mut self.world.borrow_mut(),
                &mut self.player.borrow_mut(),
            );
            if res.is_action() {
                format!(
                    "{}{}",
                    res.output(),
                    self.events().expect("There is no room.")
                )
            } else {
                res.output().to_string()
            }
        } else {
            "I do not understand that phrase.".to_string()
        }
    }

    // manages actions taken by Enemies in the current room
    fn events(&self) -> Result<String, WorldError> {
        let curr_room = &self.world.borrow().curr_room();

        if let Some(room) = self.world.borrow_mut().rooms_mut().get_mut(curr_room) {
            let mut events_str = String::new();
            let mut loot: Option<ItemMap> = None;
            for enemy in room.enemies_mut().iter_mut() {
                if enemy.1.is_angry() && enemy.1.hp() > 0 {
                    let enemy_damage = enemy.1.damage();

                    self.player.borrow_mut().take_damage(enemy_damage);
                    self.player.borrow_mut().engage_combat();

                    events_str.push_str(&format!(
                        "\nThe {} hit you for {} damage. You have {} HP left.",
                        enemy.1.name(),
                        enemy_damage,
                        self.player.borrow().hp()
                    ));
                }
                if enemy.1.hp() <= 0 {
                    self.player.borrow_mut().disengage_combat();
                    loot = Some(enemy.1.drop_loot());
                }
            }
            if let Some(loot) = loot {
                room.items_mut().extend(loot);
            }
            room.enemies_mut().retain(|_, e| e.hp() > 0);
            if self.player.borrow().hp() <= 0 {
                events_str.push_str(" You died.");
            }
            Ok(events_str)
        } else {
            Err(WorldError::NoRoom)
        }
    }
}
