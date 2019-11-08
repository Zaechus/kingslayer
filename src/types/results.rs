#[derive(Debug, PartialEq)]
pub enum Action {
    Active,
    Passive,
}

#[derive(Debug)]
pub struct CmdResult {
    action: Action,
    output: String,
}

impl CmdResult {
    pub const fn new(action: Action, output: String) -> Self {
        Self { action, output }
    }

    pub fn is_active(&self) -> bool {
        self.action == Action::Active
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn already_closed(name: &str) -> CmdResult {
        CmdResult::new(Action::Passive, format!("The {} is already closed.", name))
    }

    pub fn already_opened(name: &str) -> CmdResult {
        CmdResult::new(Action::Passive, format!("The {} is already opened.", name))
    }

    pub fn do_what(word: &str) -> CmdResult {
        CmdResult::new(Action::Passive, format!("What do you want to {}?", word))
    }

    pub fn dont_have(name: &str) -> CmdResult {
        CmdResult::new(
            Action::Passive,
            format!("You do not have the \"{}\".", name),
        )
    }

    pub fn no_comprendo() -> CmdResult {
        CmdResult::new(
            Action::Passive,
            "I do not understand that phrase.".to_owned(),
        )
    }

    pub fn no_item_here(name: &str) -> CmdResult {
        CmdResult::new(Action::Passive, format!("There is no \"{}\" here.", name))
    }

    pub fn not_container(name: &str) -> CmdResult {
        CmdResult::new(Action::Passive, format!("The {} is not a container.", name))
    }
}
