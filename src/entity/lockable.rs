use serde::{Deserialize, Serialize};

use crate::types::CmdResult;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DoorLock {
    Locked,
    Unlocked,
}

impl DoorLock {
    pub fn is_locked(self) -> bool {
        match self {
            DoorLock::Locked => true,
            DoorLock::Unlocked => false,
        }
    }
}

pub trait Lockable {
    fn unlock(&mut self) -> CmdResult;

    fn lock(&mut self) -> CmdResult;

    fn is_locked(&self) -> bool;
}
