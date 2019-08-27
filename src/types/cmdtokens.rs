use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CmdTokens {
    num_words: usize,
    verb: String,
    obj: String,
    prep: String,
    obj_prep: String,
}

impl CmdTokens {
    pub fn new(num_words: usize, verb: &str, obj: &str, prep: &str, obj_prep: &str) -> Self {
        Self {
            num_words,
            verb: verb.to_owned(),
            obj: obj.to_owned(),
            prep: prep.to_owned(),
            obj_prep: obj_prep.to_owned(),
        }
    }

    pub fn num_words(&self) -> usize {
        self.num_words
    }
    pub fn verb(&self) -> &str {
        &self.verb
    }
    pub fn obj(&self) -> &str {
        &self.obj
    }
    pub fn prep(&self) -> &str {
        &self.prep
    }
    pub fn obj_prep(&self) -> &str {
        &self.obj_prep
    }

    pub fn after_verb(&self) -> String {
        format!("{}{}{}", self.obj, self.prep, self.obj_prep)
    }
    pub fn after_verb_vec(&self) -> Vec<String> {
        self.after_verb()
            .split_whitespace()
            .map(str::to_string)
            .collect()
    }
}
