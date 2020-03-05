use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use super::{
    Closeable, Element, Enemy, Entity,
    Item::{self, Container},
    Lockable, Pathway,
};
use crate::types::{Action, Allies, CmdResult, Elements, Enemies, Items, Paths};

// A section of the world connected by paths
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Room {
    name: String,
    desc: String,
    paths: Paths,
    #[serde(default)]
    enemies: Enemies,
    #[serde(default)]
    allies: Allies,
    #[serde(default)]
    elements: Elements,
    #[serde(default)]
    items: Items,
}

impl Room {
    // collects all descriptions of entities in the Room for printing
    pub fn long_desc(&self) -> String {
        let mut desc = format!("{}\n{}", self.name, self.desc);
        for el in self.elements.iter() {
            desc.push_str(&format!("\n{}", el.desc()));
        }
        for path in self.paths.iter() {
            if !path.desc().is_empty() {
                desc.push_str(&format!("\n{}", path.long_desc()));
            }
        }
        for item in self.items.iter() {
            desc.push_str(&format!("\n{}", item.long_desc()));
        }
        for ally in self.allies.iter() {
            desc.push_str(&format!("\n{}", ally.desc()));
        }
        for enemy in self.enemies.iter() {
            desc.push_str(&format!("\n{}", enemy.long_desc()));
        }
        desc
    }

    fn similar_item_pos(&self, name: &str) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            self.items
                .iter()
                .position(|item| item.name().split_whitespace().any(|word| word == name))
        } else {
            self.items
                .par_iter()
                .position_any(|item| item.name().par_split_whitespace().any(|word| word == name))
        }
    }
    #[allow(clippy::borrowed_box)]
    pub fn find_similar_element(&self, name: &str) -> Option<&Box<Element>> {
        if cfg!(target_arch = "wasm32") {
            self.elements.iter().find(|el| {
                el.name().split_whitespace().any(|el_word| {
                    name.split_whitespace()
                        .any(|name_word| name_word == el_word)
                })
            })
        } else {
            self.elements.par_iter().find_any(|el| {
                el.name().par_split_whitespace().any(|el_word| {
                    name.par_split_whitespace()
                        .any(|name_word| name_word == el_word)
                })
            })
        }
    }
    fn similar_enemy_pos(&self, name: &str) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            self.enemies
                .iter()
                .position(|item| item.name().split_whitespace().any(|word| word == name))
        } else {
            self.enemies
                .par_iter()
                .position_any(|item| item.name().par_split_whitespace().any(|word| word == name))
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_path(&self, direction: &str) -> Option<&Box<Pathway>> {
        if cfg!(target_arch = "wasm32") {
            self.paths
                .iter()
                .find(|path| path.directions().iter().any(|d| d == direction))
        } else {
            self.paths.par_iter().find_any(|path| {
                path.directions()
                    .par_iter()
                    .position_any(|d| d == direction)
                    .is_some()
            })
        }
    }
    #[allow(clippy::borrowed_box)]
    pub fn get_path_mut(&mut self, direction: &str) -> Option<&mut Box<Pathway>> {
        if cfg!(target_arch = "wasm32") {
            self.paths
                .iter_mut()
                .find(|path| path.directions().iter().any(|d| d == direction))
        } else {
            self.paths.par_iter_mut().find_any(|path| {
                path.directions()
                    .par_iter()
                    .position_any(|d| d == direction)
                    .is_some()
            })
        }
    }

    pub fn drain_all(&mut self) -> Items {
        self.items.drain(0..).collect()
    }

    // take an Item from a container Item in the current Room
    pub fn give_from(
        &mut self,
        item_name: &str,
        container_name: &str,
    ) -> Result<Box<Item>, CmdResult> {
        if let Some(container) = self.item_find_mut(container_name) {
            if let Container(ref mut container) = **container {
                if container.is_closed() {
                    Err(CmdResult::new(
                        Action::Active,
                        format!("The {} is closed.", container_name),
                    ))
                } else if let Some(item) = container.position(item_name) {
                    Ok(container.remove(item))
                } else {
                    Err(CmdResult::new(
                        Action::Passive,
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
            if let Some(container) = self.item_find_mut(container_name) {
                if let Container(ref mut container) = **container {
                    if container.is_closed() {
                        (
                            CmdResult::new(
                                Action::Active,
                                format!("The {} is closed.", container_name),
                            ),
                            Some(item),
                        )
                    } else {
                        container.push(item);
                        (CmdResult::new(Action::Active, "Placed.".to_owned()), None)
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

    pub fn unlock(&mut self, name: &str) -> CmdResult {
        if let Some(path) = self.get_path_mut(name) {
            if path.is_locked() {
                path.unlock()
            } else {
                CmdResult::already_unlocked(name)
            }
        } else {
            CmdResult::no_item_here(name)
        }
    }

    pub fn open(&mut self, name: &str) -> CmdResult {
        if let Some(path) = self.get_path_mut(name) {
            if path.is_locked() {
                CmdResult::new(Action::Active, "The way is locked.".to_owned())
            } else if path.is_closed() {
                path.open()
            } else {
                CmdResult::already_opened(name)
            }
        } else if let Some(item) = self.item_find_mut(name) {
            if let Container(ref mut container) = **item {
                container.open()
            } else {
                CmdResult::not_container(name)
            }
        } else if let Some(item) = self.similar_item_pos(name) {
            if let Some(item) = self.items.get_mut(item) {
                if let Container(ref mut item) = **item {
                    item.open()
                } else {
                    CmdResult::not_container(name)
                }
            } else {
                CmdResult::no_item_here(name)
            }
        } else {
            CmdResult::no_item_here(name)
        }
    }

    pub fn close(&mut self, name: &str) -> CmdResult {
        if let Some(path) = self.get_path_mut(name) {
            if path.is_closed() {
                CmdResult::already_closed(name)
            } else {
                path.close();
                CmdResult::new(Action::Active, "Closed.".to_owned())
            }
        } else if let Some(item) = self.item_find_mut(name) {
            if let Container(ref mut item) = **item {
                item.close()
            } else {
                CmdResult::not_container(name)
            }
        } else if let Some(item) = self.similar_item_pos(name) {
            if let Some(item) = self.items.get_mut(item) {
                if let Container(ref mut item) = **item {
                    item.close()
                } else {
                    CmdResult::not_container(name)
                }
            } else {
                CmdResult::no_item_here(name)
            }
        } else {
            CmdResult::no_item_here(name)
        }
    }

    // interact with an Ally
    pub fn hail(&self, _ally_name: &str) -> CmdResult {
        // if let Some(_ally) = self.allies.iter().find(|x| x.name() == ally_name) {
        CmdResult::new(Action::Passive, "Hail, friend.".to_owned())
        // } else {
        //     CmdResult::no_item_here(ally_name)
        // }
    }

    pub fn harm(
        &mut self,
        enemy: usize,
        damage: Option<u32>,
        enemy_name: &str,
        weapon: &str,
    ) -> CmdResult {
        if let Some(enemy) = self.enemies.get_mut(enemy) {
            if let Some(damage) = damage {
                if let Some(res) = enemy.get_hit(damage) {
                    res
                } else if enemy.is_alive() {
                    CmdResult::new(
                        Action::Active,
                        format!(
                            "You hit the {} with your {} for {} damage.",
                            enemy_name, weapon, damage,
                        ),
                    )
                } else {
                    let mut res = format!(
                        "You hit the {} with your {} for {} damage. It is dead.\n",
                        enemy_name, weapon, damage
                    );
                    if !enemy.loot().is_empty() {
                        res.push_str("It dropped:\n");
                        for loot in enemy.loot() {
                            res.push_str(&format!(" {},", loot.long_name()));
                        }
                    }
                    if cfg!(target_arch = "wasm32") {
                        self.items.extend(enemy.drop_loot());
                    } else {
                        self.items.par_extend(enemy.drop_loot());
                    }
                    CmdResult::new(Action::Active, res)
                }
            } else {
                CmdResult::dont_have(weapon)
            }
        } else {
            CmdResult::no_item_here(enemy_name)
        }
    }

    pub fn harm_enemy(&mut self, damage: Option<u32>, enemy_name: &str, weapon: &str) -> CmdResult {
        if let Some(enemy) = self.enemy_pos(enemy_name) {
            self.harm(enemy, damage, enemy_name, weapon)
        } else if let Some(enemy) = self.similar_enemy_pos(enemy_name) {
            self.harm(enemy, damage, enemy_name, weapon)
        } else {
            CmdResult::no_item_here(enemy_name)
        }
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if let Some(item) = self.item_find(name) {
            Some(CmdResult::new(Action::Active, item.inspect().to_owned()))
        } else if let Some(item) = self.find_similar_element(name) {
            Some(CmdResult::new(Action::Active, item.inspect().to_owned()))
        } else if let Some(pathway) = self.get_path(name) {
            Some(CmdResult::new(Action::Active, pathway.inspect().to_owned()))
        } else if let Some(enemy) = self.enemy_find(name) {
            Some(CmdResult::new(Action::Active, enemy.inspect().to_owned()))
        } else if let Some(ally) = self.allies.iter().find(|x| x.name() == name) {
            Some(CmdResult::new(Action::Active, ally.inspect().to_owned()))
        } else {
            None
        }
    }

    pub fn take_item(&mut self, name: &str, item: Option<Box<Item>>) -> CmdResult {
        if let Some(item) = item {
            self.items.push(item);
            CmdResult::new(Action::Active, "Dropped.".to_owned())
        } else {
            CmdResult::dont_have(name)
        }
    }

    pub fn remove_item(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.item_pos(name) {
            Some(self.items.remove(item))
        } else if let Some(item) = self.similar_item_pos(name) {
            Some(self.items.remove(item))
        } else {
            None
        }
    }

    pub fn add_element(&mut self, el: Element) {
        self.elements.push(Box::new(el));
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(Box::new(item));
    }

    pub fn spawn_enemy(&mut self, enemy: Enemy) {
        self.enemies.push(Box::new(enemy));
    }

    pub const fn enemies(&self) -> &Enemies {
        &self.enemies
    }
    pub fn enemies_mut(&mut self) -> &mut Enemies {
        &mut self.enemies
    }

    fn item_pos(&self, name: &str) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            self.items.iter().position(|x| x.name() == name)
        } else {
            self.items.par_iter().position_any(|x| x.name() == name)
        }
    }
    #[allow(clippy::borrowed_box)]
    fn item_find(&self, name: &str) -> Option<&Box<Item>> {
        if cfg!(target_arch = "wasm32") {
            self.items.iter().find(|x| x.name() == name)
        } else {
            self.items.par_iter().find_any(|x| x.name() == name)
        }
    }
    #[allow(clippy::borrowed_box)]
    fn item_find_mut(&mut self, name: &str) -> Option<&mut Box<Item>> {
        if cfg!(target_arch = "wasm32") {
            self.items.iter_mut().find(|x| x.name() == name)
        } else {
            self.items.par_iter_mut().find_any(|x| x.name() == name)
        }
    }

    fn enemy_pos(&self, name: &str) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            self.enemies.iter().position(|x| x.name() == name)
        } else {
            self.enemies.par_iter().position_any(|x| x.name() == name)
        }
    }
    #[allow(clippy::borrowed_box)]
    fn enemy_find(&self, name: &str) -> Option<&Box<Enemy>> {
        if cfg!(target_arch = "wasm32") {
            self.enemies.iter().find(|x| x.name() == name)
        } else {
            self.enemies.par_iter().find_any(|x| x.name() == name)
        }
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
