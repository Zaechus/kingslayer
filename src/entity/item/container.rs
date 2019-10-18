use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use super::Item;
use crate::entity::{Closeable, Entity};
use crate::types::{CmdResult, Items};

// An object to be interacted with by the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    name: String,
    desc: String,
    inspect: String,
    is_closed: bool,
    is_locked: bool,
    contents: Items,
}

impl Container {
    pub fn long_name(&self) -> String {
        if !self.contents.is_empty() && !self.is_closed {
            let mut desc = format!("{}; it contains:", self.name);
            for item in self.contents.iter() {
                desc = format!("{}\n    {}", desc, item.name());
            }
            desc
        } else {
            self.name.to_owned()
        }
    }

    pub fn long_desc(&self) -> String {
        if !self.contents.is_empty() && !self.is_closed {
            let mut desc = format!("{}\nThe {} contains:", self.desc, self.name);
            for item in self.contents.iter() {
                desc = format!("{}\n  {}", desc, item.name());
            }
            desc
        } else {
            self.desc.to_owned()
        }
    }

    pub fn find_similar_item(&self, name: &str) -> Option<usize> {
        self.contents
            .par_iter()
            .position_any(|item| item.name().par_split_whitespace().any(|word| word == name))
    }

    pub fn position(&self, name: &str) -> Option<usize> {
        self.contents.par_iter().position_any(|x| x.name() == name)
    }

    pub fn push(&mut self, item: Box<Item>) {
        self.contents.push(item);
    }
    pub fn remove(&mut self, item: usize) -> Box<Item> {
        self.contents.remove(item)
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
        if self.is_closed {
            self.is_closed = false;
            CmdResult::new(true, "Opened.".to_owned())
        } else {
            CmdResult::already_opened(&self.name)
        }
    }

    fn close(&mut self) -> CmdResult {
        if self.is_closed {
            CmdResult::already_closed(&self.name)
        } else {
            self.is_closed = true;
            CmdResult::new(true, "Closed.".to_owned())
        }
    }

    fn is_closed(&self) -> bool {
        self.is_closed
    }
}
