use crate::types::CmdResult;

pub fn already_closed(name: &str) -> CmdResult {
    CmdResult::new(false, format!("The {} already closed.", name))
}

pub fn already_opened(name: &str) -> CmdResult {
    CmdResult::new(false, format!("The {} already opened.", name))
}

pub fn do_what(word: &str) -> CmdResult {
    CmdResult::new(false, format!("What do you want to {}?", word))
}

pub fn dont_have(name: &str) -> CmdResult {
    CmdResult::new(false, format!("You do not have the \"{}\".", name))
}

pub fn no_item_here(name: &str) -> CmdResult {
    CmdResult::new(false, format!("There is no \"{}\" here.", name))
}

pub fn not_container(name: &str) -> CmdResult {
    CmdResult::new(false, format!("The {} is not a container.", name))
}
