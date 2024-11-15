use serde::{Deserialize, Serialize};

use crate::container::Container;

#[derive(Debug, Deserialize, Serialize)]
enum Nature {
    Aggressive,
    Inanimate,
    Passive,
}

impl Default for Nature {
    fn default() -> Self {
        Self::Inanimate
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum Food {
    Edible,
    Poisonous,
    Not,
}

impl Default for Food {
    fn default() -> Self {
        Self::Not
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum Opacity {
    Opaque,
    Transparent,
}

impl Default for Opacity {
    fn default() -> Self {
        Self::Opaque
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct Item {
    nature: Nature,
    can_take: bool,
    close_message: String,
    container: Container,
    covering: Vec<String>,
    damage: i8,
    desc: String,
    dest: String,
    details: String,
    door: String,
    food: Food,
    go_message: String,
    hp: i8,
    locations: Vec<String>,
    move_message: String,
    moved_message: String,
    names: Vec<String>,
    opacity: Opacity,
    open_message: String,
    take_message: String,
}

impl Item {
    pub(crate) const fn is_aggressive(&self) -> bool {
        matches!(self.nature, Nature::Aggressive)
    }

    pub(crate) const fn can_eat(&self) -> bool {
        !matches!(self.food, Food::Not)
    }

    pub(crate) fn close(&mut self) -> String {
        match self.container {
            Container::Open => {
                self.container = Container::Closed;
                if self.close_message.is_empty() {
                    "Closed.".to_owned()
                } else {
                    self.close_message.clone()
                }
            }
            Container::Closed => {
                format!("The {} is already closed.", self.name())
            }
            _ => format!("You cannot do that to a {}.", self.name()),
        }
    }

    pub(crate) const fn container(&self) -> &Container {
        &self.container
    }

    pub(crate) const fn damage(&self) -> i8 {
        self.damage
    }

    pub(crate) fn desc(&self) -> &str {
        &self.desc
    }

    pub(crate) fn dest(&self) -> &str {
        &self.dest
    }

    pub(crate) fn details(&self) -> &str {
        &self.details
    }

    pub(crate) fn door(&self) -> &str {
        &self.door
    }

    pub(crate) fn go_message(&self) -> &str {
        if self.go_message.is_empty() {
            "Nice try."
        } else {
            &self.go_message
        }
    }

    pub(crate) fn hp(&self) -> i8 {
        self.hp
    }

    pub(crate) fn hurt(&mut self, damage: i8) {
        if matches!(self.nature, Nature::Passive) {
            self.nature = Nature::Aggressive;
        }
        self.hp = self.hp.saturating_sub(damage)
    }

    pub(crate) const fn is_clear(&self) -> bool {
        matches!(self.container, Container::Open | Container::True)
            || matches!(self.opacity, Opacity::Transparent)
    }

    pub(crate) const fn is_closed(&self) -> bool {
        matches!(self.container, Container::Closed)
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
        if let Some(location) = self.locations.first() {
            location
        } else {
            ""
        }
    }

    pub(crate) fn move_self(&mut self) -> Result<(String, Vec<String>), String> {
        if !self.covering.is_empty() {
            Ok((
                if self.move_message.is_empty() {
                    "Done.".to_owned()
                } else {
                    self.move_message.clone()
                },
                self.covering.drain(..).collect(),
            ))
        } else {
            Err(if !self.moved_message.is_empty() {
                self.moved_message.clone()
            } else if self.can_take {
                format!("Moving the {} doesn't do anything.", self.name())
            } else {
                format!("You cannot move the {}.", self.name())
            })
        }
    }

    pub(crate) fn name(&self) -> &str {
        if let Some(name) = self.names.first() {
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

    pub(crate) fn set_location(&mut self, location: String) {
        self.locations = vec![location];
    }

    pub(crate) fn take(&mut self, location: &str) -> &str {
        if self.can_take {
            self.locations = vec![location.to_owned()];
            "Taken."
        } else if !self.take_message.is_empty() {
            &self.take_message
        } else {
            "Nice try."
        }
    }

    pub(crate) fn try_take(&self) -> bool {
        self.can_take || !self.take_message.is_empty()
    }
}
