#[derive(Debug)]
pub struct CmdResult {
    is_action: bool,
    output: String,
}

impl CmdResult {
    pub fn new(is_action: bool, output: String) -> Self {
        Self { is_action, output }
    }

    pub fn is_action(&self) -> bool {
        self.is_action
    }

    pub fn output(&self) -> &String {
        &self.output
    }
}
