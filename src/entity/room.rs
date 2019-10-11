use serde::{Deserialize, Serialize};

use crate::entity::Entity;
use crate::types::{AllyMap, EnemyMap, ItemMap, PathMap};

// A section of the world connected by paths
#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    #[serde(default)]
    name: String,
    desc: String,
    paths: PathMap,
    enemies: EnemyMap,
    allies: AllyMap,
    items: ItemMap,
}

impl Room {
    // collects all descriptions of entities in the Room for printing
    pub fn long_desc(&self) -> String {
        let mut desc = format!("{}\n{}", self.name, self.desc);
        for path in self.paths.values() {
            desc.push_str(&format!("\n{}", path.desc()));
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

    pub fn paths(&self) -> &PathMap {
        &self.paths
    }
    pub fn paths_mut(&mut self) -> &mut PathMap {
        &mut self.paths
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
