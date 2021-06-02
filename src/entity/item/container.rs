use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use super::Item;
use crate::{
    entity::{Closeable, Entity, Opening},
    types::{Action, CmdResult, Items},
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    name: String,
    desc: String,
    inspect: String,
    opening: Opening,
    contents: Items,
}

impl Container {
    pub fn long_name(&self) -> String {
        if !self.contents.is_empty() && self.opening.is_open() {
            self.contents
                .iter()
                .fold(format!("{}; it contains:", self.name), |desc, item| {
                    format!("{}\n    {}", desc, item.name())
                })
        } else {
            self.name.to_owned()
        }
    }

    pub fn long_desc(&self) -> String {
        if !self.contents.is_empty() && self.opening.is_open() {
            self.contents.iter().fold(
                format!("{}\nThe {} contains:", self.desc, self.name),
                |desc, item| format!("{}\n  {}", desc, item.name()),
            )
        } else {
            self.desc.to_owned()
        }
    }

    fn item_pos(&self, item_name: &str) -> Option<usize> {
        if cfg!(target_arch = "wasm32") {
            self.contents
                .iter()
                .map(|item| item.name().split_whitespace().collect())
                .position(|direction: Vec<&str>| {
                    item_name
                        .split_whitespace()
                        .all(|ref word| direction.contains(word))
                })
        } else {
            self.contents
                .par_iter()
                .map(|item| item.name().par_split_whitespace().collect())
                .position_any(|direction: Vec<&str>| {
                    item_name
                        .par_split_whitespace()
                        .all(|ref word| direction.contains(word))
                })
        }
    }

    pub fn push_item(&mut self, item: Box<Item>) {
        self.contents.push(item);
    }

    pub fn give_item(&mut self, item_name: &str) -> Result<Box<Item>, CmdResult> {
        if self.is_closed() {
            Err(CmdResult::new(
                Action::Active,
                format!("The {} is closed.", self.name),
            ))
        } else if let Some(pos) = self.item_pos(item_name) {
            Ok(self.contents.remove(pos))
        } else {
            Err(CmdResult::new(
                Action::Passive,
                format!("There is no \"{}\" in the {}.", item_name, self.name),
            ))
        }
    }
}

impl Entity for Container {
    fn name(&self) -> &str {
        &self.name
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn inspect(&self) -> &str {
        &self.inspect
    }
}

impl Closeable for Container {
    fn open(&mut self) -> CmdResult {
        if self.opening.is_closed() {
            self.opening = Opening::Open;
            CmdResult::new(Action::Active, "Opened.")
        } else {
            CmdResult::already_opened(&self.name)
        }
    }

    fn close(&mut self) -> CmdResult {
        if self.opening.is_open() {
            self.opening = Opening::Closed;
            CmdResult::new(Action::Active, "Closed.")
        } else {
            CmdResult::already_closed(&self.name)
        }
    }

    fn is_closed(&self) -> bool {
        self.opening.is_closed()
    }
}
