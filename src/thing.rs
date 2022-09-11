use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game::PLAYER;

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct Thing {
    names: Vec<String>,
    desc: String,
    what: String,
    #[serde(default = "rooms")]
    locations: Vec<String>,
    dest: String,
    can_take: bool,
    container: Container,
    door: String,
    take_message: String,
    go_message: String,
}

fn rooms() -> Vec<String> {
    vec![String::new()]
}

impl Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl Thing {
    pub(crate) fn can_open(&self) -> bool {
        matches!(self.container, Container::Open | Container::Closed)
    }

    pub(crate) fn container(&self) -> &Container {
        &self.container
    }

    pub(crate) fn desc(&self) -> &str {
        &self.desc
    }

    pub(crate) fn dest(&self) -> &str {
        &self.dest
    }

    pub(crate) fn go_message(&self) -> &str {
        &self.go_message
    }

    pub(crate) fn is_container(&self) -> bool {
        !matches!(self.container, Container::False)
    }

    pub(crate) fn is_in(&self, location: &str) -> bool {
        self.locations.iter().any(|l| l == location)
    }

    pub(crate) fn is_open(&self) -> bool {
        matches!(self.container, Container::Open | Container::True)
    }

    pub(crate) fn location(&self) -> &str {
        &self.locations[0]
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

    pub(crate) fn open(&mut self) {
        self.container = Container::Open;
    }

    pub(crate) fn set_location(&mut self, location: String) {
        self.locations = vec![location];
    }

    pub(crate) fn take(&mut self) -> &str {
        if self.can_take {
            self.locations[0] = PLAYER.to_owned();
            "Taken."
        } else if self.take_message.is_empty() {
            "You cannot take that."
        } else {
            &self.take_message
        }
    }

    pub(crate) fn what(&self) -> &str {
        &self.what
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) enum Container {
    Open,
    Closed,
    True,
    False,
}

impl Default for Container {
    fn default() -> Self {
        Self::False
    }
}
