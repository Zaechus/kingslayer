use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

    pub fn with_obj(mut self, obj: Option<String>) -> Self {
        self.obj = obj;
        self
    }

    pub fn with_obj_prep(mut self, obj: Option<String>) -> Self {
        self.obj_prep = obj;
        self
    }

    pub fn short_verb(&self) -> (Option<&str>, Option<&str>) {
        if let Some(verb) = &self.verb {
            if verb.len() >= 6 {
                (Some(&verb), Some(&verb[0..6]))
            } else {
                (Some(&verb), Some(&verb))
            }
        } else {
            (None, None)
        }
    }
    pub fn verb(&self) -> Option<&str> {
        if let Some(verb) = &self.verb {
            Some(&verb)
        } else {
            None
        }
    }
    pub fn verb_clone(&self) -> Option<String> {
        self.verb.clone()
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
}
