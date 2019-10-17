use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{
    entity::{
        Closeable, Entity,
        Item::{self, Container, Weapon},
    },
    types::{CmdResult, Items},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    items: Items,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: Items::new(),
        }
    }

    pub fn find_similar_item(&self, name: &str) -> Option<usize> {
        self.items
            .par_iter()
            .position_any(|item| item.name().par_split_whitespace().any(|word| word == name))
    }

    pub fn close(&mut self, item_name: &str) -> Option<CmdResult> {
        if let Some(item) = self.find_mut(item_name) {
            if let Container(item) = &mut **item {
                Some(item.close())
            } else {
                Some(CmdResult::not_container(item_name))
            }
        } else if let Some(item) = self.find_similar_item(item_name) {
            if let Some(item) = self.items.get_mut(item) {
                if let Container(item) = &mut **item {
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
        self.items.par_iter().find_any(|x| x.name() == name)
    }
    #[allow(clippy::borrowed_box)]
    pub fn find_mut(&mut self, name: &str) -> Option<&mut Box<Item>> {
        self.items.par_iter_mut().find_any(|x| x.name() == name)
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
                if let Container(container) = &mut **container {
                    if container.is_closed() {
                        self.items.push(item);
                        CmdResult::new(true, format!("The {} is closed.", container_name))
                    } else {
                        container.push(item);
                        CmdResult::new(true, "Placed.".to_owned())
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
            if let Container(item) = &mut **item {
                Some(item.open())
            } else {
                Some(CmdResult::not_container(item_name))
            }
        } else if let Some(item) = self.find_similar_item(item_name) {
            if let Some(item) = self.items.get_mut(item) {
                if let Container(item) = &mut **item {
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
        self.items.par_iter().position_any(|x| x.name() == name)
    }

    pub fn print(&self) -> String {
        let mut items_carried = String::new();
        if self.items.is_empty() {
            items_carried.push_str("Your inventory is empty.");
        } else {
            items_carried.push_str("You are carrying:");
            for item in self.items.iter() {
                items_carried = format!("{}\n  {}", items_carried, item.long_name());
            }
        }
        items_carried
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
        if let Some(obj) = item {
            let mut res = String::from("Taken.");
            if let Weapon(_) = *obj {
                res.push_str("\n(You can equip weapons with \"equip\" or \"draw\")");
            }
            self.items.push(obj);
            CmdResult::new(true, res)
        } else {
            CmdResult::no_item_here(name)
        }
    }

    pub fn take_all(&mut self, items: Items) -> CmdResult {
        if items.is_empty() {
            CmdResult::new(false, "There is nothing to take.".to_owned())
        } else {
            let times = items.len();
            self.items.par_extend(items);

            let mut res = String::new();
            for _ in 0..times {
                res.push_str("Taken. ");
            }
            CmdResult::new(true, res)
        }
    }

    fn take_out_of(&mut self, item_name: &str, container_name: &str) -> Option<Box<Item>> {
        if let Some(container) = self.find_mut(container_name) {
            if let Container(container) = &mut **container {
                if let Some(item) = container.position(item_name) {
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

    pub fn take_from(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        if let Some(item) = self.take_out_of(item_name, container_name) {
            self.items.push(item);
            CmdResult::new(true, "Taken.".to_owned())
        } else {
            CmdResult::dont_have(container_name)
        }
    }
}
