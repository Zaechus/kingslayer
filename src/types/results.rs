pub struct CmdResult {
    is_action: bool,
    command: String,
}

impl CmdResult {
    pub fn new(is_action: bool, command: String) -> Self {
        Self { is_action, command }
    }

    pub fn is_action(&self) -> bool {
        self.is_action
    }

    pub fn command(&self) -> &String {
        &self.command
    }
}
