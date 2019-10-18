use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use super::{
    Closeable, Entity,
    Item::{self, Container},
};
use crate::types::{Allies, CmdResult, Enemies, Items, PathMap};

// A section of the world connected by paths
#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    name: String,
    desc: String,
    paths: PathMap,
    #[serde(default)]
    enemies: Enemies,
    #[serde(default)]
    allies: Allies,
    #[serde(default)]
    items: Items,
}

impl Room {
    // collects all descriptions of entities in the Room for printing
    pub fn long_desc(&self) -> String {
        let mut desc = format!("{}\n{}", self.name, self.desc);
        for path in self.paths.values() {
            desc.push_str(&format!("\n{}", path.long_desc()));
        }
        for enemy in self.enemies.iter() {
            desc.push_str(&format!("\n{}", enemy.desc()));
        }
        for ally in self.allies.iter() {
            desc.push_str(&format!("\n{}", ally.desc()));
        }
        for item in self.items.iter() {
            desc.push_str(&format!("\n{}", item.long_desc()));
        }
        desc
    }

    fn find_similar_item(&self, name: &str) -> Option<usize> {
        self.items
            .par_iter()
            .position_any(|item| item.name().par_split_whitespace().any(|word| word == name))
    }

    pub fn drain_all(&mut self) -> Items {
        self.items.drain(0..).collect()
    }

    pub fn extend_items(&mut self, new_items: Items) {
        self.items.par_extend(new_items);
    }

    // take an Item from a container Item in the current Room
    pub fn give_from(
        &mut self,
        item_name: &str,
        container_name: &str,
    ) -> Result<Box<Item>, CmdResult> {
        if let Some(container) = self
            .items
            .par_iter_mut()
            .find_any(|x| x.name() == container_name)
        {
            if let Container(ref mut container) = **container {
                if container.is_closed() {
                    Err(CmdResult::new(
                        true,
                        format!("The {} is closed.", container_name),
                    ))
                } else if let Some(item) = container.position(item_name) {
                    Ok(container.remove(item))
                } else {
                    Err(CmdResult::new(
                        false,
                        format!("There is no \"{}\" in the {}.", item_name, container_name),
                    ))
                }
            } else {
                Err(CmdResult::not_container(container_name))
            }
        } else {
            Err(CmdResult::no_item_here(container_name))
        }
    }

    // insert an Item into a container Item in the current Room
    pub fn insert_into(
        &mut self,
        item_name: &str,
        container_name: &str,
        item: Option<Box<Item>>,
    ) -> (CmdResult, Option<Box<Item>>) {
        if let Some(item) = item {
            if let Some(container) = self
                .items
                .par_iter_mut()
                .find_any(|x| x.name() == container_name)
            {
                if let Container(ref mut container) = **container {
                    if container.is_closed() {
                        (
                            CmdResult::new(true, format!("The {} is closed.", container_name)),
                            Some(item),
                        )
                    } else {
                        container.push(item);
                        (CmdResult::new(true, "Placed.".to_owned()), None)
                    }
                } else {
                    (CmdResult::not_container(container_name), Some(item))
                }
            } else {
                (CmdResult::no_item_here(container_name), Some(item))
            }
        } else {
            (CmdResult::dont_have(item_name), None)
        }
    }

    pub fn open(&mut self, name: &str) -> CmdResult {
        if let Some(path) = self.paths.get_mut(name) {
            if path.is_closed() {
                path.open();
                CmdResult::new(true, "Opened.".to_owned())
            } else {
                CmdResult::already_opened(name)
            }
        } else if let Some(item) = self.items.par_iter_mut().find_any(|x| x.name() == name) {
            if let Container(ref mut container) = **item {
                container.open()
            } else {
                CmdResult::not_container(name)
            }
        } else {
            CmdResult::no_item_here(name)
        }
    }

    pub fn close(&mut self, name: &str) -> CmdResult {
        if let Some(path) = self.paths.get_mut(name) {
            if path.is_closed() {
                CmdResult::already_closed(name)
            } else {
                path.close();
                CmdResult::new(true, "Closed.".to_owned())
            }
        } else if let Some(item) = self.items.par_iter_mut().find_any(|x| x.name() == name) {
            if let Container(ref mut container) = **item {
                container.close()
            } else {
                CmdResult::not_container(name)
            }
        } else {
            CmdResult::no_item_here(name)
        }
    }

    // interact with an Ally
    pub fn hail(&self, ally_name: &str) -> CmdResult {
        if let Some(_ally) = self.allies.par_iter().find_any(|x| x.name() == ally_name) {
            CmdResult::new(false, "TODO: interact with ally".to_owned())
        } else {
            CmdResult::no_item_here(ally_name)
        }
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if let Some(item) = self.items.par_iter().find_any(|x| x.name() == name) {
            Some(CmdResult::new(true, item.inspect().to_owned()))
        } else if let Some(pathway) = self.paths.get(name) {
            Some(CmdResult::new(true, pathway.inspect().to_owned()))
        } else if let Some(enemy) = self.enemies.par_iter().find_any(|x| x.name() == name) {
            Some(CmdResult::new(true, enemy.inspect().to_owned()))
        } else if let Some(ally) = self.allies.par_iter().find_any(|x| x.name() == name) {
            Some(CmdResult::new(true, ally.inspect().to_owned()))
        } else {
            None
        }
    }

    pub fn take_item(&mut self, name: &str, item: Option<Box<Item>>) -> CmdResult {
        if let Some(item) = item {
            self.items.push(item);
            CmdResult::new(true, "Dropped.".to_owned())
        } else {
            CmdResult::dont_have(name)
        }
    }

    pub fn remove_item(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.items.par_iter().position_any(|x| x.name() == name) {
            Some(self.items.remove(item))
        } else if let Some(item) = self.find_similar_item(name) {
            Some(self.items.remove(item))
        } else {
            None
        }
    }

    pub fn paths(&self) -> &PathMap {
        &self.paths
    }

    pub fn enemies(&self) -> &Enemies {
        &self.enemies
    }
    pub fn enemies_mut(&mut self) -> &mut Enemies {
        &mut self.enemies
    }
}

impl Entity for Room {
    fn name(&self) -> &str {
        &self.name
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn inspect(&self) -> &str {
        &self.desc
    }
}
