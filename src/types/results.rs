use serde::{Deserialize, Serialize};

use crate::input::CmdTokens;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Action {
    Active,
    Passive,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CmdResult {
    action: Action,
    output: String,
    request_input: Option<CmdTokens>,
}

impl Default for CmdResult {
    fn default() -> Self {
        Self {
            action: Action::Passive,
            output: String::new(),
            request_input: None,
        }
    }
}

impl CmdResult {
    pub fn new<S>(action: Action, output: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            action,
            output: output.into(),
            request_input: None,
        }
    }

    pub fn with_request_input(mut self, cmd: CmdTokens) -> Self {
        self.request_input = Some(cmd);
        self
    }

    pub fn has_request(&self) -> bool {
        self.request_input.is_some()
    }

    pub fn request_input(&self) -> Option<CmdTokens> {
        self.request_input.clone()
    }

    pub fn is_active(&self) -> bool {
        self.action == Action::Active
    }

    pub fn succeeded(&self) -> bool {
        self.action != Action::Failed
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn already_unlocked(name: &str) -> CmdResult {
        CmdResult::new(Action::Failed, format!("The {} is already unlocked.", name))
    }

    pub fn is_locked(name: &str) -> CmdResult {
        CmdResult::new(
            Action::Passive,
            format!("The {} is locked. I wonder if I can pick it...", name),
        )
    }

    pub fn already_closed(name: &str) -> CmdResult {
        CmdResult::new(Action::Failed, format!("The {} is already closed.", name))
    }

    pub fn already_opened(name: &str) -> CmdResult {
        CmdResult::new(Action::Failed, format!("The {} is already opened.", name))
    }

    pub fn do_what(verb: &str) -> CmdResult {
        CmdResult::new(Action::Passive, format!("What do you want to {}?", verb))
            .with_request_input(CmdTokens::new(verb))
    }

    pub fn do_what_prep(verb: &str, prep: Option<&str>, obj_prep: Option<&str>) -> CmdResult {
        if let (Some(prep), Some(obj_prep)) = (prep, obj_prep) {
            CmdResult::new(
                Action::Passive,
                format!("What do you want to {} {} the {}?", verb, prep, obj_prep),
            )
            .with_request_input(CmdTokens::new(verb).with_prep(prep).with_obj_prep(obj_prep))
        } else {
            CmdResult::do_what(verb)
        }
    }

    pub fn dont_have(name: &str) -> CmdResult {
        CmdResult::new(Action::Failed, format!("You do not have the \"{}\".", name))
    }

    pub fn no_comprendo() -> CmdResult {
        CmdResult::new(Action::Failed, "I do not understand that phrase.")
    }

    pub fn no_item_here(name: &str) -> CmdResult {
        CmdResult::new(Action::Failed, format!("There is no \"{}\" here.", name))
    }

    pub fn not_container(name: &str) -> CmdResult {
        CmdResult::new(Action::Failed, format!("The {} is not a container.", name))
    }
}
