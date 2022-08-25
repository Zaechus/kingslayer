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

    pub fn close(&mut self, item_name: &str) -> Option<CmdResult> {
        if let Some(item) = self.find_item_mut(item_name) {
            if let Container(ref mut item) = item {
                Some(item.close())
            } else {
                Some(CmdResult::not_container(item_name))
            }
        } else {
            None
        }
    }

    pub fn item_pos(&self, item_name: &str) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            self.items
                .iter()
                .map(|item| item.name().split_whitespace().collect())
                .position(|item: Vec<&str>| {
                    item_name
                        .split_whitespace()
                        .all(|ref word| item.contains(word))
                })
        } else {
            self.items
                .par_iter()
                .map(|item| item.name().par_split_whitespace().collect())
                .position_any(|item: Vec<&str>| {
                    item_name
                        .par_split_whitespace()
                        .all(|ref word| item.contains(word))
                })
        }
    }

    pub fn find_item(&self, item_name: &str) -> Option<&Item> {
        if let Some(pos) = self.item_pos(item_name) {
            self.items.get(pos).map(|x| &**x)
        } else {
            None
        }
    }
    pub fn find_item_mut(&mut self, item_name: &str) -> Option<&mut Item> {
        if let Some(pos) = self.item_pos(item_name) {
            self.items.get_mut(pos).map(|x| &mut **x)
        } else {
            None
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.find_item(name).is_some()
    }

    pub fn insert_into(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        let item = self.remove_item(item_name);

        if let Some(item) = item {
            if let Some(container) = self.find_item_mut(container_name) {
                if let Container(ref mut container) = container {
                    if container.is_closed() {
                        self.items.push(item);
                        CmdResult::new(Action::Active, format!("The {} is closed.", container_name))
                    } else {
                        container.push_item(item);
                        CmdResult::new(Action::Active, "Placed.")
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
        if let Some(item) = self.find_item_mut(item_name) {
            if let Container(ref mut item) = item {
                Some(item.open())
            } else {
                Some(CmdResult::not_container(item_name))
            }
        } else {
            None
        }
    }

    pub fn print(&self) -> String {
        if self.items.is_empty() {
            String::from("Your inventory is empty.")
        } else {
            self.items.iter().fold(
                format!("Gold: {}\nYou are carrying:", self.gold),
                |res, item| format!("{}\n  {}", res, item.long_name()),
            )
        }
    }

    pub fn push(&mut self, item: Box<Item>) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item_name: &str) -> Option<Box<Item>> {
        if let Some(item) = self.item_pos(item_name) {
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
            CmdResult::new(Action::Passive, "There is nothing to take.")
        } else {
            let times = items.len();
            if cfg!(target_arch = "wasm32") {
                self.items.extend(items);
            } else {
                self.items.par_extend(items);
            }

            let mut gold = 0;
            self.items.retain(|x| {
                if let Gold(ref g) = **x {
                    gold += g.amount();
                    false
                } else {
                    true
                }
            });
            self.gold += gold;
            CmdResult::new(
                Action::Active,
                (0..times).fold(String::new(), |res, _| format!("{}Taken. ", res)),
            )
        }
    }

    fn take_out_of(
        &mut self,
        item_name: &str,
        container_name: &str,
    ) -> Result<Box<Item>, CmdResult> {
        if let Some(container) = self.find_item_mut(container_name) {
            if let Container(ref mut container) = container {
                container.give_item(item_name)
            } else {
                Err(CmdResult::not_container(container_name))
            }
        } else {
            Err(CmdResult::dont_have(container_name))
        }
    }

    pub fn take_from_self(&mut self, item_name: &str, container_name: &str) -> CmdResult {
        match self.take_out_of(item_name, container_name) {
            Ok(item) => {
                self.items.push(item);
                CmdResult::new(Action::Active, String::from("Taken."))
            }
            Err(res) => res,
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
