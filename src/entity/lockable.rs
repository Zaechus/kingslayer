use serde::{Deserialize, Serialize};

use crate::types::CmdResult;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Lock {
    Locked,
    Unlocked,
}

impl Lock {
    pub fn is_locked(self) -> bool {
        match self {
            Lock::Locked => true,
            Lock::Unlocked => false,
        }
    }
}

pub trait Lockable {
    fn unlock(&mut self) -> CmdResult;

    fn lock(&mut self) -> CmdResult;

    fn is_locked(&self) -> bool;
}
