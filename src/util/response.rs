use crate::types::CmdResult;

pub fn do_what(word: &str) -> CmdResult {
    CmdResult::new(false, format!("What do you want to {}?", word))
}

pub fn dont_have(name: &str) -> CmdResult {
    CmdResult::new(false, format!("You do not have the \"{}\".", name))
}
