use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct Item {
    can_take: bool,
    close_message: String,
    container: Container,
    desc: String,
    dest: String,
    door: String,
    go_message: String,
    locations: Vec<String>,
    names: Vec<String>,
    open_message: String,
    take_message: String,
    what: String,
}

impl Item {
    pub(crate) fn close(&mut self) -> String {
        match self.container {
            Container::Closed => {
                format!("The {} is already closed.", self.name())
            }
            Container::Open => {
                self.container = Container::Closed;
                if self.close_message.is_empty() {
                    "Closed.".to_owned()
                } else {
                    self.close_message.clone()
                }
            }
            _ => format!("You cannot do that to a {}.", self.name()),
        }
    }

    pub(crate) fn open(&mut self, reveals: String) -> String {
        match self.container {
            Container::Open => {
                format!("The {} is already open.", self.name())
            }
            Container::Closed => {
                self.container = Container::Open;
                if !self.open_message.is_empty() {
                    self.open_message.clone()
                } else if !reveals.is_empty() {
                    format!("Opening the {} reveals {}.", self.name(), reveals)
                } else {
                    "Opened.".to_owned()
                }
            }
            _ => format!("You cannot do that to a {}.", self.name()),
        }
    }

    pub(crate) const fn container(&self) -> &Container {
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

    pub(crate) const fn is_container(&self) -> bool {
        !matches!(self.container, Container::False)
    }

    pub(crate) fn is_in(&self, location: &str) -> bool {
        self.locations.iter().any(|l| l == location)
    }

    pub(crate) const fn is_open(&self) -> bool {
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

    pub(crate) fn set_location(&mut self, location: String) {
        self.locations = vec![location];
    }

    pub(crate) fn take(&mut self, location: &str) -> &str {
        if self.can_take {
            self.locations = vec![location.to_owned()];
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

pub(crate) fn list_items(items: &[&Item]) -> String {
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
