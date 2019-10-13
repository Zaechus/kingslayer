use serde::{Deserialize, Serialize};

use super::{
    Closeable, Entity,
    Item::{self, Container},
};
use crate::{
    response::{already_closed, already_opened, no_item_here, not_container},
    types::{AllyMap, CmdResult, EnemyMap, ItemMap, PathMap},
};

// A section of the world connected by paths
#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    name: String,
    desc: String,
    paths: PathMap,
    #[serde(default)]
    enemies: EnemyMap,
    #[serde(default)]
    allies: AllyMap,
    #[serde(default)]
    items: ItemMap,
}

impl Room {
    // collects all descriptions of entities in the Room for printing
    pub fn long_desc(&self) -> String {
        let mut desc = format!("{}\n{}", self.name, self.desc);
        for path in self.paths.values() {
            desc.push_str(&format!("\n{}", path.long_desc()));
        }
        for enemy in self.enemies.values() {
            desc.push_str(&format!("\n{}", enemy.desc()));
        }
        for ally in self.allies.values() {
            desc.push_str(&format!("\n{}", ally.desc()));
        }
        for item in self.items.values() {
            desc.push_str(&format!("\n{}", item.long_desc()));
        }
        desc
    }

    fn find_similar_item_name(&self, name: &str) -> Option<&String> {
        self.items
            .keys()
            .find(|key| key.split_whitespace().any(|word| word == name))
    }

    pub fn open(&mut self, name: &str) -> CmdResult {
        if let Some(path) = self.paths.get_mut(name) {
            if path.is_closed() {
                path.open();
                CmdResult::new(true, "Opened.".to_owned())
            } else {
                already_opened(name)
            }
        } else if let Some(item) = self.items.get_mut(name) {
            if let Container(container) = &mut **item {
                container.open()
            } else {
                not_container(name)
            }
        } else {
            no_item_here(name)
        }
    }

    pub fn close(&mut self, name: &str) -> CmdResult {
        if let Some(path) = self.paths.get_mut(name) {
            if path.is_closed() {
                already_closed(name)
            } else {
                path.close();
                CmdResult::new(true, "Closed.".to_owned())
            }
        } else if let Some(item) = self.items.get_mut(name) {
            if let Container(container) = &mut **item {
                container.close()
            } else {
                not_container(name)
            }
        } else {
            no_item_here(name)
        }
    }

    pub fn remove_item(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.items.remove(name) {
            Some(item)
        } else {
            let similar_name = if let Some(s) = self.find_similar_item_name(name) {
                s.clone()
            } else {
                return None;
            };
            self.items.remove(&similar_name)
        }
    }

    pub fn paths(&self) -> &PathMap {
        &self.paths
    }

    pub fn items(&self) -> &ItemMap {
        &self.items
    }
    pub fn items_mut(&mut self) -> &mut ItemMap {
        &mut self.items
    }

    pub fn enemies(&self) -> &EnemyMap {
        &self.enemies
    }
    pub fn enemies_mut(&mut self) -> &mut EnemyMap {
        &mut self.enemies
    }

    pub fn allies(&self) -> &AllyMap {
        &self.allies
    }
}

impl Entity for Room {
    fn name(&self) -> &String {
        &self.name
    }

    fn desc(&self) -> &String {
        &self.desc
    }

    fn inspect(&self) -> &String {
        &self.desc
    }
}
