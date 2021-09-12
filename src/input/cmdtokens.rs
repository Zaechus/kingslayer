use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CmdTokens {
    verb: Option<String>,
    obj: Option<String>,
    prep: Option<String>,
    obj_prep: Option<String>,
}

impl CmdTokens {
    pub fn new<S>(verb: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            verb: Some(verb.into()),
            obj: None,
            prep: None,
            obj_prep: None,
        }
    }

    pub fn with_obj<S>(mut self, obj: S) -> Self
    where
        S: Into<String>,
    {
        self.obj = Some(obj.into());
        self
    }

    pub fn with_prep<S>(mut self, prep: S) -> Self
    where
        S: Into<String>,
    {
        self.prep = Some(prep.into());
        self
    }

    pub fn with_obj_prep<S>(mut self, obj_prep: S) -> Self
    where
        S: Into<String>,
    {
        self.obj_prep = Some(obj_prep.into());
        self
    }

    pub fn from<OS>(verb: OS, obj: OS, prep: OS, obj_prep: OS) -> Self
    where
        OS: Into<Option<String>>,
    {
        Self {
            verb: verb.into(),
            obj: obj.into(),
            prep: prep.into(),
            obj_prep: obj_prep.into(),
        }
    }

    pub fn short_verb(&self) -> (Option<&str>, Option<&str>) {
        if let Some(verb) = &self.verb {
            if verb.len() >= 6 {
                (Some(verb), Some(&verb[0..6]))
            } else {
                (Some(verb), Some(verb))
            }
        } else {
            (None, None)
        }
    }
    pub fn verb(&self) -> Option<&str> {
        self.verb.as_deref()
    }
    pub fn obj(&self) -> Option<&str> {
        self.obj.as_deref()
    }
    pub fn prep(&self) -> Option<&str> {
        self.prep.as_deref()
    }
    pub fn obj_prep(&self) -> Option<&str> {
        self.obj_prep.as_deref()
    }
}
