use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{
    entity::{
        Closeable, Entity,
        Item::{self, Container, Gold},
    },
    types::{Action, CmdResult, Items},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Inventory {
    items: Items,
    gold: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: Items::new(),
            gold: 0,
        }
    }

    pub fn find_similar_item(&self, name: &str) -> Option<usize> {
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

    pub fn close(&mut self, item_name: &str) -> Option<CmdResult> {
        if let Some(item) = self.find_mut(item_name) {
            if let Container(ref mut item) = **item {
                Some(item.close())
            } else {
                Some(CmdResult::not_container(item_name))
            }
        } else if let Some(item) = self.find_similar_item(item_name) {
            if let Some(item) = self.items.get_mut(item) {
                if let Container(ref mut item) = **item {
                    Some(item.close())
                } else {
                    Some(CmdResult::not_container(&item_name))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn find(&self, name: &str) -> Option<&Box<Item>> {
        if cfg!(target_arch = "wasm32") {
            self.items.iter().find(|x| x.name() == name)
        } else {
            self.items.par_iter().find_any(|x| x.name() == name)
        }
    }
    #[allow(clippy::borrowed_box)]
    pub fn find_mut(&mut self, name: &str) -> Option<&mut Box<Item>> {
        if cfg!(target_arch = "wasm32") {
            self.items.iter_mut().find(|x| x.name() == name)
        } else {
            self.items.par_iter_mut().find_any(|x| x.name() == name)
        }
    }
    #[allow(clippy::borrowed_box)]
    pub fn find_similar_item_mut(&mut self, name: &str) -> Option<&mut Box<Item>> {
        if cfg!(target_arch = "wasm32") {
            self.items
                .iter_mut()
                .find(|item| item.name().split_whitespace().any(|word| word == name))
        } else {
            self.items
                .par_iter_mut()
                .find_any(|item| item.name().par_split_whitespace().any(|word| word == name))
        }
    }

    pub fn has(&self, name: &str) -> bool {
        if self.find(name).is_some() {
            true
        } else {
            self.find_similar_item(name).is_some()
        }
    }

    fn inventory_remove(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.position(name) {
            Some(self.items.remove(item))
        } else {
            None
        }
    }

    pub fn insert_into(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        let item = self.inventory_remove(item_name);

        if let Some(item) = item {
            if let Some(container) = self.find_mut(container_name) {
                if let Container(ref mut container) = **container {
                    if container.is_closed() {
                        self.items.push(item);
                        CmdResult::new(Action::Active, format!("The {} is closed.", container_name))
                    } else {
                        container.push(item);
                        CmdResult::new(Action::Active, "Placed.".to_owned())
                    }
                } else {
                    self.items.push(item);
                    CmdResult::not_container(container_name)
                }
            } else {
                self.items.push(item);
                CmdResult::dont_have(container_name)
            }
        } else {
            CmdResult::dont_have(item_name)
        }
    }

    pub fn open(&mut self, item_name: &str) -> Option<CmdResult> {
        if let Some(item) = self.find_mut(item_name) {
            if let Container(ref mut item) = **item {
                Some(item.open())
            } else {
                Some(CmdResult::not_container(item_name))
            }
        } else if let Some(item) = self.find_similar_item(item_name) {
            if let Some(item) = self.items.get_mut(item) {
                if let Container(ref mut item) = **item {
                    Some(item.open())
                } else {
                    Some(CmdResult::not_container(item_name))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn position(&self, name: &str) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            self.items.iter().position(|x| x.name() == name)
        } else {
            self.items.par_iter().position_any(|x| x.name() == name)
        }
    }

    pub fn print(&self) -> String {
        if self.items.is_empty() {
            String::from("Your inventory is empty.")
        } else {
            let mut items_carried = format!("Gold: {}\nYou are carrying:", self.gold);
            items_carried.reserve(5 * self.items.len() + 8);

            for item in self.items.iter() {
                items_carried = format!("{}\n  {}", items_carried, item.long_name());
            }
            items_carried.shrink_to_fit();
            items_carried
        }
    }

    pub fn push(&mut self, item: Box<Item>) {
        self.items.push(item);
    }
    pub fn remove(&mut self, item: usize) -> Box<Item> {
        self.items.remove(item)
    }

    pub fn remove_item(&mut self, name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.position(name) {
            Some(self.items.remove(item))
        } else if let Some(item) = self.find_similar_item(name) {
            Some(self.items.remove(item))
        } else {
            None
        }
    }

    pub fn take(&mut self, name: &str, item: Option<Box<Item>>) -> CmdResult {
        if let Some(item) = item {
            if let Gold(g) = *item {
                self.gold += g.amount();
            } else {
                self.items.push(item);
            }
            CmdResult::new(Action::Active, String::from("Taken."))
        } else {
            CmdResult::no_item_here(name)
        }
    }

    pub fn take_all(&mut self, items: Items) -> CmdResult {
        if items.is_empty() {
            CmdResult::new(Action::Passive, "There is nothing to take.".to_owned())
        } else {
            let times = items.len();
            if cfg!(target_arch = "wasm32") {
                self.items.extend(items);
            } else {
                self.items.par_extend(items);
            }

            let mut res = String::with_capacity(7 * times);
            for _ in 0..times {
                res.push_str("Taken. ");
            }
            let mut gold = 0;
            self.items.retain(|x| {
                if let Gold(g) = &**x {
                    gold += g.amount();
                    false
                } else {
                    true
                }
            });
            self.gold += gold;
            CmdResult::new(Action::Active, res)
        }
    }

    fn take_out_of(&mut self, item_name: &str, container_name: &str) -> Option<Box<Item>> {
        if let Some(container) = self.find_mut(container_name) {
            if let Container(ref mut container) = **container {
                if let Some(item) = container.position(item_name) {
                    Some(container.remove(item))
                } else if let Some(item) = container.find_similar_item(item_name) {
                    Some(container.remove(item))
                } else {
                    None
                }
            } else {
                None
            }
        } else if let Some(container) = self.find_similar_item_mut(container_name) {
            if let Container(ref mut container) = **container {
                if let Some(item) = container.position(item_name) {
                    Some(container.remove(item))
                } else if let Some(item) = container.find_similar_item(item_name) {
                    Some(container.remove(item))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn take_from_self(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        if let Some(item) = self.take_out_of(item_name, container_name) {
            self.items.push(item);
            CmdResult::new(Action::Active, "Taken.".to_owned())
        } else {
            CmdResult::dont_have(container_name)
        }
    }

    pub fn take_item_from(&mut self, item: Result<Box<Item>, CmdResult>) -> CmdResult {
        match item {
            Ok(item) => {
                self.items.push(item);
                CmdResult::new(Action::Active, String::from("Taken."))
            }
            Err(res) => res,
        }
    }
}
