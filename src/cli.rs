use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

use serde_derive::{Deserialize, Serialize};

use crate::{
    input::{Lexer, Parser},
    player::Player,
    types::{CmdResult, ItemMap},
    util::read_line,
    world::World,
};

/// A command line interface for controlling interactions between objects in a game
#[derive(Debug, Serialize, Deserialize)]
pub struct Cli {
    lexer: Lexer,
    player: RefCell<Player>,
    world: RefCell<World>,
}

impl Cli {
    /// Create a Cli from a JSON file
    pub fn from_json_file(path: &str) -> Self {
        Self {
            lexer: Lexer::new(),
            player: RefCell::new(Player::new()),
            world: Cli::get_world_json(path),
        }
    }

    /// Create a Cli from a string containing JSON
    pub fn from_json_str(json: &str) -> Self {
        Self {
            lexer: Lexer::new(),
            player: RefCell::new(Player::new()),
            world: Cli::get_world_json_str(json),
        }
    }

    fn get_world_json(path: &str) -> RefCell<World> {
        let world_file = File::open(path).expect("Unable to open world file");
        let mut world_file_reader = BufReader::new(world_file);
        let mut data = String::new();
        world_file_reader
            .read_to_string(&mut data)
            .expect("Unable to read string from world file");

        serde_json::from_str(&data).expect("Error creating world from JSON file.")
    }

    fn get_world_json_str(json: &str) -> RefCell<World> {
        serde_json::from_str(json).expect("Error creating world from string.")
    }

    /// Prompts the user for input with stdin
    pub fn prompt() -> String {
        loop {
            print!("\n> ");
            io::stdout().flush().expect("Error flushing stdout");
            let input = read_line();
            if !input.is_empty() {
                return input;
            } else {
                println!("Excuse me?");
            }
        }
    }

    /// Returns a helpful list of game commands
    pub fn help() -> CmdResult {
        CmdResult::new(
            false,
            "Some available commands:
        go, enter <direction>\tmove through a listed entrance
        \tshort directions: n, s, e, w, ne, nw, se, sw, u, d
        \tlong directions:
        \t   north, south, east, west,
        \t   northeast, northwest, southeast, southwest,
        \t   up, down, (other listed entrance)\n
        take\t\tput an item from the room into your inventory
        drop\t\tdrop an item from your inventory into the room
        l, look\t\tlook around the room
        i, inventory\tprint the contents of your inventory
        x, examine\t\tshow additional information about an item
        draw, equip\t\tuse an item from your inventory as your default weapon
        don, put on\tdon a set of armor to increase your armor class
        kill\t\tattack an enemy with your main hand or a chosen weapon
        open | close\topen/close a container or pathway
        heal\t\treplenish some HP
        increase\tincrease a chosen ability score by 1 if stat points are available
        status\t\tdisplay information on the state of your character"
                .to_owned(),
        )
    }

    /// Start a basic Kingslayer game for the command line
    pub fn start(&self) {
        println!("type \"help\" to learn some common commands.\n");
        println!("Use \"increase\" to use your initial stat points.\n");
        println!("{}", self.ask("l"));
        while self.player.borrow().is_alive() {
            println!("{}", self.ask(&Cli::prompt()));
        }
    }

    /// Handle user input and return the result of commands and events
    pub fn ask(&self, input: &str) -> String {
        let command = self.lexer.lex(input);

        if !command.verb().is_empty() {
            let res = Parser::parse(
                command,
                &mut self.world.borrow_mut(),
                &mut self.player.borrow_mut(),
            );

            if res.is_action() {
                format!(
                    "{}{}",
                    res.output(),
                    self.combat(&mut self.world.borrow_mut())
                )
            } else {
                res.output().to_owned()
            }
        } else {
            "I do not understand that phrase.".to_owned()
        }
    }

    // manages actions taken by Enemies in the current room
    fn combat(&self, world: &mut World) -> String {
        let mut events_str = String::new();
        let mut loot: ItemMap = ItemMap::new();

        for enemy in world.get_curr_room_mut().enemies_mut().values_mut() {
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
        world.get_curr_room_mut().items_mut().extend(loot);
        world
            .get_curr_room_mut()
            .enemies_mut()
            .retain(|_, e| e.is_alive());

        if !self.player.borrow().is_alive() {
            events_str.push_str("\nYou died.");
        } else {
            events_str.push_str(&self.player.borrow_mut().level_up());
        }
        events_str
    }
}
