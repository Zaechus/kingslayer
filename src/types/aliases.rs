use std::collections::HashMap;

use crate::entities::Item;

pub type ItemMap = HashMap<String, Box<Item>>;
