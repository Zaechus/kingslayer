pub struct CmdResult {
    pub is_action: bool,
    pub command: String,
}

impl CmdResult {
    pub fn new(is_action: bool, command: &str) -> Self {
        Self {
            is_action,
            command: command.to_string(),
        }
    }
}
