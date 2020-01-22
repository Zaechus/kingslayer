use std::collections::HashMap;

use crate::entity::{Ally, Enemy, Item, Pathway, Room};

pub type Allies = Vec<Box<Ally>>;
pub type Enemies = Vec<Box<Enemy>>;
pub type Items = Vec<Box<Item>>;
pub type PathMap = Vec<Box<Pathway>>;
pub type RoomMap = HashMap<String, Box<Room>>;
