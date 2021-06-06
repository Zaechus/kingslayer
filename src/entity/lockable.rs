use serde::{Deserialize, Serialize};

use crate::types::CmdResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DoorLock {
    Locked(String),
    Unlocked,
}

impl DoorLock {
    pub fn is_locked(&self) -> bool {
        match self {
            DoorLock::Locked(_) => true,
            DoorLock::Unlocked => false,
        }
    }
}

pub trait Lockable {
    fn unlock(&mut self) -> CmdResult;

    fn lock(&mut self) -> CmdResult;

    fn is_locked(&self) -> bool;
}
