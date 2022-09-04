#[derive(Default)]
pub struct Tokens {
    verb: Option<String>,
    obj: Option<String>,
    prep: Option<String>,
    obj_prep: Option<String>,
}

impl Tokens {
    pub fn new(
        verb: Option<String>,
        obj: Option<String>,
        prep: Option<String>,
        obj_prep: Option<String>,
    ) -> Self {
        Self {
            verb,
            obj,
            prep,
            obj_prep,
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
