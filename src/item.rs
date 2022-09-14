use serde::{Deserialize, Serialize};

use crate::game::PLAYER;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct Item {
    can_take: bool,
    container: Container,
    desc: String,
    dest: String,
    door: String,
    go_message: String,
    locations: Vec<String>,
    names: Vec<String>,
    take_message: String,
    what: String,
}

impl Item {
    pub(crate) fn can_open(&self) -> bool {
        matches!(self.container, Container::Open | Container::Closed)
    }

    pub(crate) fn close(&mut self) -> String {
        if let Container::Closed = self.container {
            format!("The {} is already closed.", self.name())
        } else {
            self.container = Container::Closed;
            "Closed.".to_owned()
        }
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

    pub(crate) fn door(&self) -> &str {
        &self.door
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
        if let Some(location) = self.locations.get(0) {
            location
        } else {
            ""
        }
    }

    pub(crate) fn name(&self) -> &str {
        if let Some(name) = self.names.get(0) {
            name
        } else {
            ""
        }
    }

    pub(crate) fn names_contains(&self, search: &str) -> bool {
        self.names.iter().any(|name| {
            let name = name.to_lowercase();

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
            "Nice try."
        } else {
            &self.take_message
        }
    }

    pub(crate) fn what(&self) -> &str {
        &self.what
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

pub(crate) fn list_items(items: Vec<&Item>) -> String {
    match items.len() {
        0 => String::new(),
        1 => format!("a {}", items[0].name()),
        2 => format!("a {} and a {}", items[0].name(), items[1].name()),
        _ => {
            format!(
                "a {}, and a {}",
                items[1..items.len() - 1].iter().fold(
                    items[0].name().to_owned(),
                    |acc, i| format!("{}, a {}", acc, i.name())
                ),
                items[items.len() - 1].name()
            )
        }
    }
}
