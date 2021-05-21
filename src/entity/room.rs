use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use super::{
    Closeable, Element, Enemy, Entity,
    Item::{self, Container},
    Lockable, Pathway,
};
use crate::types::{Action, Allies, Attack, CmdResult, Elements, Enemies, Items, Paths};

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

    #[allow(clippy::borrowed_box)]
    fn find_element(&self, name: &str) -> Option<&Box<Element>> {
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

    fn path_pos(&self, dir_name: &str) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            self.paths
                .iter()
                .position(|pathway| pathway.any_direction(dir_name.split_whitespace().collect()))
        } else {
            self.paths.par_iter().position_any(|pathway| {
                pathway.any_direction(dir_name.par_split_whitespace().collect())
            })
        }
    }
    #[allow(clippy::borrowed_box)]
    pub fn find_path(&self, direction: &str) -> Option<&Box<Pathway>> {
        if let Some(pos) = self.path_pos(direction) {
            self.paths.get(pos)
        } else {
            None
        }
    }
    #[allow(clippy::borrowed_box)]
    pub fn find_path_mut(&mut self, direction: &str) -> Option<&mut Box<Pathway>> {
        if let Some(pos) = self.path_pos(direction) {
            self.paths.get_mut(pos)
        } else {
            None
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
        if let Some(container) = self.find_item_mut(container_name) {
            if let Container(ref mut container) = **container {
                container.give_item(item_name)
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
            if let Some(container) = self.find_item_mut(container_name) {
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
                        container.push_item(item);
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
        if let Some(path) = self.find_path_mut(name) {
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
        if let Some(path) = self.find_path_mut(name) {
            if path.is_locked() {
                CmdResult::is_locked(name)
            } else if path.is_closed() {
                path.open()
            } else {
                CmdResult::already_opened(name)
            }
        } else if let Some(item) = self.find_item_mut(name) {
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
        if let Some(path) = self.find_path_mut(name) {
            if path.is_closed() {
                CmdResult::already_closed(name)
            } else {
                path.close();
                CmdResult::new(Action::Active, "Closed.".to_owned())
            }
        } else if let Some(item) = self.find_item_mut(name) {
            if let Container(ref mut item) = **item {
                item.close()
            } else {
                CmdResult::not_container(name)
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

    fn harm(&mut self, enemy: usize, enemy_name: &str, attack: Attack) -> CmdResult {
        if let Some(enemy) = self.enemies.get_mut(enemy) {
            if let Some(damage) = attack.damage() {
                if let Some(res) = enemy.take_damage(damage) {
                    res
                } else if enemy.is_alive() {
                    CmdResult::new(
                        Action::Active,
                        format!(
                            "You hit the {} with your {} for {} damage.",
                            enemy_name,
                            attack.weapon_name(),
                            damage,
                        ),
                    )
                } else {
                    let mut res = format!(
                        "You hit the {} with your {} for {} damage. It is dead.\n",
                        enemy_name,
                        attack.weapon_name(),
                        damage
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
                CmdResult::dont_have(&attack.weapon_name())
            }
        } else {
            CmdResult::no_item_here(enemy_name)
        }
    }

    pub fn harm_enemy(&mut self, enemy_name: &str, attack: Attack) -> CmdResult {
        if let Some(enemy) = self.enemy_pos(enemy_name) {
            self.harm(enemy, enemy_name, attack)
        } else if let Some(enemy) = self.similar_enemy_pos(enemy_name) {
            self.harm(enemy, enemy_name, attack)
        } else {
            CmdResult::no_item_here(enemy_name)
        }
    }

    pub fn inspect(&self, name: &str) -> Option<CmdResult> {
        if let Some(item) = self.find_item(name) {
            Some(CmdResult::new(Action::Active, item.inspect().to_owned()))
        } else if let Some(item) = self.find_element(name) {
            Some(CmdResult::new(Action::Active, item.inspect().to_owned()))
        } else if let Some(pathway) = self.find_path(name) {
            Some(CmdResult::new(Action::Active, pathway.inspect().to_owned()))
        } else if let Some(enemy) = self.find_enemy(name) {
            Some(CmdResult::new(Action::Active, enemy.inspect().to_owned()))
        } else {
            self.allies
                .iter()
                .find(|x| x.name() == name)
                .map(|ally| CmdResult::new(Action::Active, ally.inspect().to_owned()))
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
        if let Some(item) = self.item_pos(if cfg!(target_arch = "wasm32") {
            name.split_whitespace().collect()
        } else {
            name.par_split_whitespace().collect()
        }) {
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

    fn item_pos(&self, item_name: Vec<&str>) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            self.items
                .iter()
                .map(|item| item.name().split_whitespace().collect())
                .position(|item: Vec<&str>| item_name.iter().all(|ref word| item.contains(word)))
        } else {
            self.items
                .par_iter()
                .map(|item| item.name().par_split_whitespace().collect())
                .position_any(|item: Vec<&str>| {
                    item_name.par_iter().all(|ref word| item.contains(word))
                })
        }
    }
    #[allow(clippy::borrowed_box)]
    fn find_item(&self, name: &str) -> Option<&Box<Item>> {
        if let Some(pos) = self.item_pos(if cfg!(target_arch = "wasm32") {
            name.split_whitespace().collect()
        } else {
            name.par_split_whitespace().collect()
        }) {
            self.items.get(pos)
        } else {
            None
        }
    }
    #[allow(clippy::borrowed_box)]
    fn find_item_mut(&mut self, name: &str) -> Option<&mut Box<Item>> {
        if let Some(pos) = self.item_pos(if cfg!(target_arch = "wasm32") {
            name.split_whitespace().collect()
        } else {
            name.par_split_whitespace().collect()
        }) {
            self.items.get_mut(pos)
        } else {
            None
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
    fn find_enemy(&self, name: &str) -> Option<&Box<Enemy>> {
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
