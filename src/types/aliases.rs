use std::collections::HashMap;

use crate::entity::{Ally, Enemy, Item, Pathway, Room};

pub type AllyMap = HashMap<String, Box<Ally>>;
pub type EnemyMap = HashMap<String, Box<Enemy>>;
pub type ItemMap = HashMap<String, Box<Item>>;
pub type PathMap = HashMap<String, Box<Pathway>>;
pub type RoomMap = HashMap<String, Box<Room>>;
