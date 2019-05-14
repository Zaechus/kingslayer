use std::collections::HashMap;

use crate::entity::{Enemy, Item, Pathway, Room};

pub type ItemMap = HashMap<String, Box<Item>>;
pub type EnemyMap = HashMap<String, Box<Enemy>>;
pub type RoomMap = HashMap<String, Box<Room>>;
pub type PathMap = HashMap<String, Box<Pathway>>;
