use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use super::{Closeable, DoorLock, Durability, Entity, Lockable, Opening};
use crate::{
    dice_roll,
    types::{Action, CmdResult},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pathway {
    directions: Vec<String>,
    target: String,
    #[serde(default)]
    desc: String,
    #[serde(default)]
    inspect: String,
    opening: Option<Opening>,
    lock: Option<DoorLock>,
    #[serde(default)]
    durability: Durability,
}

impl Pathway {
    pub fn long_desc(&self) -> String {
        if let Some(opening) = self.opening {
            match opening {
                Opening::Open => format!("{} The way is open.", self.desc),
                Opening::Closed => format!("{} The way is shut.", self.desc),
            }
        } else {
            self.desc.to_owned()
        }
    }

    pub fn any_direction(&self, dir_name: &str) -> bool {
        if cfg!(target_arch = "wasm32") {
            self.directions
                .iter()
                .map(|direction| direction.split_whitespace().collect())
                .any(|direction: Vec<&str>| {
                    dir_name
                        .par_split_whitespace()
                        .all(|ref word| direction.contains(word))
                })
        } else {
            self.directions
                .par_iter()
                .map(|direction| direction.par_split_whitespace().collect())
                .any(|direction: Vec<&str>| {
                    dir_name
                        .par_split_whitespace()
                        .all(|ref word| direction.contains(word))
                })
        }
    }
}

impl Entity for Pathway {
    fn name(&self) -> &str {
        &self.target
    }

    fn desc(&self) -> &str {
        &self.desc
    }

    fn inspect(&self) -> &str {
        &self.inspect
    }
}

impl Closeable for Pathway {
    fn open(&mut self) -> CmdResult {
        if let Some(opening) = self.opening {
            if opening.is_closed() {
                self.opening = Some(Opening::Open);
                CmdResult::new(Action::Active, "Opened.")
            } else {
                CmdResult::new(Action::Passive, String::from("The way is already open."))
            }
        } else {
            CmdResult::new(Action::Passive, String::from("The way is already open."))
        }
    }

    fn close(&mut self) -> CmdResult {
        if let Some(opening) = self.opening {
            if opening.is_open() {
                self.opening = Some(Opening::Closed);
                CmdResult::new(Action::Active, "Closed.")
            } else {
                CmdResult::new(Action::Passive, String::from("The way cannot be closed."))
            }
        } else {
            CmdResult::new(Action::Passive, String::from("The way cannot be closed."))
        }
    }

    fn is_closed(&self) -> bool {
        if let Some(opening) = self.opening {
            opening.is_closed()
        } else {
            false
        }
    }
}

impl Lockable for Pathway {
    fn unlock(&mut self) -> CmdResult {
        if let Some(lock) = &self.lock {
            if lock.is_locked() {
                if dice_roll(1, 20) > 10 {
                    self.lock = Some(DoorLock::Unlocked);
                    CmdResult::new(Action::Active, "Unlocked.")
                } else {
                    CmdResult::new(
                        Action::Active,
                        "You failed to pick the lock. This is harder than it looks.",
                    )
                }
            } else {
                CmdResult::new(
                    Action::Passive,
                    String::from("The way is already unlocked."),
                )
            }
        } else {
            CmdResult::new(
                Action::Passive,
                String::from("The way is already unlocked."),
            )
        }
    }

    fn lock(&mut self) -> CmdResult {
        CmdResult::new(Action::Passive, "TODO: lock it")
    }

    fn is_locked(&self) -> bool {
        if let Some(lock) = &self.lock {
            lock.is_locked()
        } else {
            false
        }
    }
}
