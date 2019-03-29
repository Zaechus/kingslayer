use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

use serde_derive::{Deserialize, Serialize};

use crate::{
    input::{Lexer, Parser},
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
        CmdResult::new(
            false,
            "Some available commands:
            \tgo, enter <direction>\tmove in through a listed entrance
            \t\tshort directions: n, s, e, w, ne, nw, se, sw, u, d
            \t\tlong directions:
            \t\t   north, south, east, west,
            \t\t   northeast, northwest, southeast, southwest,
            \t\t   up, down, other listed entrance\n
            \ttake\t\tput an item from the room into your inventory
            \tdrop\t\tdrop an item from your inventory into the room
            \tl, look\t\tlook around the room
            \ti, inventory\tprint the contents of your inventory
            \tx, examine\tshow additional information about an item
            \tequip\t\tuse an item from your inventory as your default weapon
            \tkill\t\tattack an enemy with your main hand or a chosen weapon
            \topen | close\topen/close a pathway
            \trest\t\treplenish some HP
            \tincrease\t\tincrease a chosen ability score by 1 if stat points are available
            \tstatus\t\tdisplay information on the state of your character"
                .to_string(),
        )
    }

    pub fn start(&self) {
        println!("Type \"help\" to learn come commands.\n");
        println!("Use \"increase\" to use your initial stat points.\n");
        println!("{}", self.ask("l"));
        while self.player.borrow().is_alive() {
            println!("{}", self.ask(&Cli::prompt()));
        }
    }

    // handle user input
    pub fn ask(&self, input: &str) -> String {
        let command = self.lexer.lex(input);

        if !command.is_empty() {
            let res = Parser::parse(
                &command,
                &mut self.world.borrow_mut(),
                &mut self.player.borrow_mut(),
            );

            if res.is_action() {
                format!(
                    "{}{}",
                    res.output(),
                    self.combat().expect("There is no room.")
                )
            } else {
                res.output().to_string()
            }
        } else {
            "I do not understand that phrase.".to_string()
        }
    }

    // manages actions taken by Enemies in the current room
    fn combat(&self) -> Result<String, WorldError> {
        let curr_room = &self.world.borrow().curr_room();

        if let Some(room) = self.world.borrow_mut().rooms_mut().get_mut(curr_room) {
            let mut events_str = String::new();
            let mut loot: ItemMap = ItemMap::new();

            for enemy in room.enemies_mut().values() {
                if enemy.is_angry() && enemy.is_alive() {
                    let enemy_damage = enemy.damage();

                    events_str.push_str(
                        &self
                            .player
                            .borrow_mut()
                            .take_damage(enemy.name(), enemy_damage),
                    );

                    self.player.borrow_mut().engage_combat();
                }
                if !enemy.is_alive() {
                    events_str.push_str(&format!("\nYou gained {} XP.\n", enemy.xp()));
                    self.player.borrow_mut().disengage_combat();
                    self.player.borrow_mut().gain_xp(enemy.xp());
                    loot.extend(enemy.drop_loot());
                }
            }
            room.items_mut().extend(loot);
            room.enemies_mut().retain(|_, e| e.is_alive());

            if !self.player.borrow().is_alive() {
                events_str.push_str("\nYou died.");
            } else {
                events_str.push_str(&self.player.borrow_mut().level_up());
            }
            Ok(events_str)
        } else {
            Err(WorldError::NoRoom)
        }
    }
}
