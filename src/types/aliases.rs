use std::collections::HashMap;

use crate::entity::{Ally, Element, Enemy, Item, Pathway, Room};

pub type Allies = Vec<Box<Ally>>;
pub type Elements = Vec<Box<Element>>;
pub type Enemies = Vec<Box<Enemy>>;
pub type Items = Vec<Box<Item>>;
pub type Paths = Vec<Box<Pathway>>;
pub type Rooms = HashMap<String, Box<Room>>;
