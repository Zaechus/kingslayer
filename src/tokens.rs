#[derive(Debug, Default)]
pub(crate) struct Tokens {
    verb: Option<String>,
    obj: Option<String>,
    prep: Option<String>,
    obj_prep: Option<String>,
}

impl Tokens {
    pub(crate) const fn new(
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

    pub(crate) fn verb(&self) -> Option<&str> {
        self.verb.as_deref()
    }

    pub(crate) fn obj(&self) -> Option<&str> {
        self.obj.as_deref()
    }

    pub(crate) fn prep(&self) -> Option<&str> {
        self.prep.as_deref()
    }

    pub(crate) fn obj_prep(&self) -> Option<&str> {
        self.obj_prep.as_deref()
    }
}
