use serde_derive::{Deserialize, Serialize};

use crate::types::{EnemyMap, ItemMap, PathMap};

// A section of the world connected by paths
#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    name: String,
    desc: String,
    paths: PathMap,
    enemies: EnemyMap,
    items: ItemMap,
}

impl Room {
    // collects all descriptions of entities in the Room for printing
    pub fn desc(&self) -> String {
        let mut desc = format!("{}\n{}", self.name, self.desc);
        for x in self.paths.iter() {
            desc.push_str(&format!("\n{}", &(x.1).desc()));
        }
        for x in self.enemies.iter() {
            desc.push_str(&format!("\n{}", &x.1.desc()));
        }
        for x in self.items.iter() {
            desc.push_str(&format!("\n{}", &x.1.desc()));
        }
        desc
    }

    pub fn paths(&self) -> &PathMap {
        &self.paths
    }
    pub fn paths_mut(&mut self) -> &mut PathMap {
        &mut self.paths
    }
    pub fn has_path(&self, path: &str) -> bool {
        self.paths.contains_key(path)
    }

    pub fn enemies(&self) -> &EnemyMap {
        &self.enemies
    }
    pub fn enemies_mut(&mut self) -> &mut EnemyMap {
        &mut self.enemies
    }

    pub fn items(&self) -> &ItemMap {
        &self.items
    }
    pub fn items_mut(&mut self) -> &mut ItemMap {
        &mut self.items
    }
    pub fn has_item(&self, item: &str) -> bool {
        self.items.contains_key(item)
    }
}
