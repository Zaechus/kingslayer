use serde::{Deserialize, Serialize};

use crate::entity::{Closeable, Entity};
use crate::types::ItemMap;

// An object to be interacted with by the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    name: String,
    desc: String,
    inspect: String,
    is_closed: bool,
    is_locked: bool,
    contents: ItemMap,
}

impl Container {
    pub fn long_name(&self) -> String {
        if !self.contents.is_empty() && !self.is_closed {
            let mut desc = format!("{}; it contains:", self.name);
            for item in self.contents.values() {
                desc = format!("{}\n    {}", desc, item.name());
            }
            desc
        } else {
            self.name.clone()
        }
    }

    pub fn long_desc(&self) -> String {
        if !self.contents.is_empty() && !self.is_closed {
            let mut desc = format!("{}\nThe {} contains:", self.desc, self.name);
            for item in self.contents.values() {
                desc = format!("{}\n  {}", desc, item.name());
            }
            desc
        } else {
            self.desc.clone()
        }
    }

    pub fn contents_mut(&mut self) -> &mut ItemMap {
        &mut self.contents
    }
}

impl Entity for Container {
    fn name(&self) -> &String {
        &self.name
    }

    fn desc(&self) -> &String {
        &self.desc
    }

    fn inspect(&self) -> &String {
        &self.inspect
    }
}

impl Closeable for Container {
    fn open(&mut self) {
        self.is_closed = false
    }

    fn close(&mut self) {
        self.is_closed = true
    }

    fn is_closed(&self) -> bool {
        self.is_closed
    }
}
