use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

use serde::{Deserialize, Serialize};

use crate::{
    entity::Entity,
    input::{read_line, Lexer, Parser},
    player::Player,
    types::{CmdResult, ItemMap},
    world::World,
};

/// The Cli type provides a simple way to interface into the mechanics of Kingslayer with custom worlds
#[derive(Debug, Serialize, Deserialize)]
pub struct Cli {
    lexer: Lexer,
    player: RefCell<Player>,
    world: RefCell<World>,
}

impl Cli {
    /// Construct from a RON file
    pub fn from_ron_file(path: &str) -> Self {
        Self {
            lexer: Lexer::new(),
            player: RefCell::new(Player::new()),
            world: Cli::get_world_ron(path),
        }
    }

    /// Construct from a string containing RON
    pub fn from_ron_str(json: &str) -> Self {
        Self {
            lexer: Lexer::new(),
            player: RefCell::new(Player::new()),
            world: ron::de::from_str(json).expect("Error creating world from string."),
        }
    }

    /// Construct from a JSON file
    pub fn from_json_file(path: &str) -> Self {
        Self {
            lexer: Lexer::new(),
            player: RefCell::new(Player::new()),
            world: Cli::get_world_json(path),
        }
    }

    /// Construct from a string containing JSON
    pub fn from_json_str(json: &str) -> Self {
        Self {
            lexer: Lexer::new(),
            player: RefCell::new(Player::new()),
            world: serde_json::from_str(json).expect("Error creating world from string."),
        }
    }

    fn get_world_ron(path: &str) -> RefCell<World> {
        let world_file = File::open(path).expect("Unable to open world file");
        let mut world_file_reader = BufReader::new(world_file);
        let mut data = String::new();
        world_file_reader
            .read_to_string(&mut data)
            .expect("Unable to read string from world file");

        ron::de::from_str(&data).expect("Error creating world from RON file.")
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

    /// Prompts the user for input from stdin
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
            "Typical format: <action> [object] [preposition] [object]
    some prepositions: in, inside, from, on, with

Some available commands:

    Explore around the world
        go, enter       move in a direction or through a listed entrance
            Directions can be short like:
                n, s, e, w, ne, nw, se, sw, u, d
            or long like:
               north, south, east, west,
               northeast, northwest, southeast, southwest,
               up, down, (any other listed entrance)
        
        l, look         look around the room
        open | close    open/close an item or pathway

    Manipulate items found in the world
        take            put an item from the room into your inventory
        drop            drop an item from your inventory into the room
        i, inventory    print the contents of your inventory
        x, examine      show additional information about an item
        draw, equip     use an item from your inventory as your default weapon
        don, put on     don a set of armor to increase your armor class
        kill            attack an enemy with your main hand or a chosen weapon

    Manage your character
        heal            replenish some HP
        increase        increase a chosen ability score by 1 if stat points are available
        c | stats          display information on the state of your character"
                .to_owned(),
        )
    }

    /// Start a typical game for the command line
    pub fn start(&self) {
        println!("Type \"help\" if you are unfamiliar with text-based games.\n");
        println!("Use \"increase\" to use your initial stat points.\n");
        println!("{}", self.ask("l"));
        while self.player.borrow().is_alive() {
            println!("{}", self.ask(&Cli::prompt()));
        }
    }

    /// Handle user input and return the results of commands and events
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
        let mut loot = ItemMap::new();

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
