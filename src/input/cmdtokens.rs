use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CmdTokens {
    num_words: usize,
    verb: Option<String>,
    obj: Option<String>,
    prep: Option<String>,
    obj_prep: Option<String>,
}

impl CmdTokens {
    pub const fn new(
        num_words: usize,
        verb: Option<String>,
        obj: Option<String>,
        prep: Option<String>,
        obj_prep: Option<String>,
    ) -> Self {
        Self {
            num_words,
            verb,
            obj,
            prep,
            obj_prep,
        }
    }

    pub fn short_verb(&self) -> Option<&str> {
        if let Some(verb) = &self.verb {
            if verb.len() >= 6 {
                Some(&verb[0..6])
            } else {
                Some(&verb)
            }
        } else {
            None
        }
    }
    pub fn verb(&self) -> Option<&String> {
        self.verb.as_ref()
    }
    pub fn obj(&self) -> Option<&String> {
        self.obj.as_ref()
    }
    pub fn prep(&self) -> Option<&String> {
        self.prep.as_ref()
    }
    pub fn obj_prep(&self) -> Option<&String> {
        self.obj_prep.as_ref()
    }

    pub fn after_verb(&self) -> String {
        format!(
            "{}{}{}",
            self.obj.as_ref().unwrap_or(&String::new()),
            self.prep.as_ref().unwrap_or(&String::new()),
            self.obj_prep.as_ref().unwrap_or(&String::new())
        )
    }
}
