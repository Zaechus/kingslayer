use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Thing {
    names: Vec<String>,
    desc: String,
    location: String,
    dest: Option<String>,
    can_take: bool,
    take_message: String,
}

impl Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl Thing {
    pub fn new<S: Into<String>>(desc: S) -> Self {
        Self {
            names: Vec::new(),
            desc: desc.into(),
            location: String::new(),
            dest: None,
            can_take: false,
            take_message: String::new(),
        }
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.names.push(name.into());
        self
    }

    pub fn with_location<S: Into<String>>(mut self, location: S) -> Self {
        self.location = location.into();
        self
    }

    pub fn with_dest<S: Into<String>>(mut self, dest: S) -> Self {
        self.dest = Some(dest.into());
        self
    }

    pub fn with_take(mut self) -> Self {
        self.can_take = true;
        self
    }

    pub(crate) fn name(&self) -> &str {
        &self.names[0]
    }

    pub(crate) fn names_contains(&self, search: &str) -> bool {
        self.names.iter().any(|name| {
            search.split_whitespace().all(|search_word| {
                name.split_whitespace()
                    .any(|name_word| name_word == search_word)
            })
        })
    }

    pub(crate) fn desc(&self) -> &str {
        &self.desc
    }

    pub(crate) fn location(&self) -> &str {
        &self.location
    }

    pub(crate) fn set_location(&mut self, location: String) {
        self.location = location;
    }

    pub(crate) fn dest(&self) -> Option<&str> {
        self.dest.as_deref()
    }

    pub(crate) fn take(&mut self) -> &str {
        if self.can_take {
            self.location = "player".to_owned();
            "Taken."
        } else if self.take_message.is_empty() {
            "You cannot take that."
        } else {
            &self.take_message
        }
    }
}
