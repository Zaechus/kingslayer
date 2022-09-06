use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::entity::{
    open::{Closeable, Opening},
    Entity,
};

use super::{item_index, Item};

impl Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc())?;
        if !self.is_closed() && !self.contents.is_empty() {
            write!(f, " It contains:")?;
            for i in self.contents.iter() {
                write!(f, "\n  {}", i.name())?;
            }
        }
        Ok(())
    }
}

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Container {
    name: String,
    desc: String,
    inspect: String,
    #[serde(default)]
    lid: Option<Opening>,
    #[serde(default)]
    contents: Vec<Item>,
}

impl Container {
    pub(crate) fn give(&mut self, item_name: &str) -> Result<Item, String> {
        if !self.is_closed() {
            match item_index(&self.contents, item_name) {
                Some(pos) => Ok(self.contents.remove(pos)),
                None => Err(format!(
                    "There is no \"{}\" in the {}.",
                    item_name, self.name
                )),
            }
        } else {
            Err("It's closed.".to_owned())
        }
    }

    pub(crate) fn receive(&mut self, item: Result<Item, String>) -> String {
        match item {
            Ok(item) => {
                self.contents.push(item);
                "Placed.".to_owned()
            }
            Err(message) => message,
        }
    }
}

impl Closeable for Container {
    fn open(&mut self) -> String {
        if let Some(Opening::Closed) = self.lid {
            self.lid = Some(Opening::Open);
            "Opened.".to_owned()
        } else {
            "It is already open!".to_owned()
        }
    }

    fn close(&mut self) -> String {
        match self.lid {
            Some(Opening::Open) => {
                self.lid = Some(Opening::Closed);
                "Closed.".to_owned()
            }
            Some(Opening::Closed) => "It is already closed!".to_owned(),
            None => "You cannot close that.".to_owned(),
        }
    }

    fn is_closed(&self) -> bool {
        matches!(self.lid, Some(Opening::Closed))
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
