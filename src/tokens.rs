const USELESS_WORDS: [&str; 15] = [
    "a", "am", "an", "across", "around", "at", "for", "is", "of", "my", "that", "the", "this",
    "through", "to",
];
const PREPOSITIONS: [&str; 6] = ["in", "inside", "from", "on", "under", "with"];

#[derive(Clone, Debug)]
pub(crate) struct Tokens {
    verb: Option<String>,
    noun: Option<String>,
    prep: Option<String>,
    obj: Option<String>,
}

impl Tokens {
    pub(crate) fn new(input: String) -> Self {
        let mut words: Vec<_> = input
            .split_whitespace()
            .map(str::to_lowercase)
            .map(alias)
            .filter(|w| !USELESS_WORDS.contains(&w.as_str()))
            .collect();

        if let Some(prep_pos) = words
            .iter()
            .position(|w| PREPOSITIONS.contains(&w.as_str()))
        {
            if prep_pos == 0 {
                Self {
                    verb: None,
                    noun: None,
                    prep: Some(words[0].to_owned()),
                    obj: Some(words[1..].join(" ")),
                }
            } else {
                let noun = words[1..prep_pos].join(" ");
                let obj = words[prep_pos + 1..].join(" ");

                Self {
                    verb: Some(words[0].to_owned()),
                    noun: if noun.is_empty() { None } else { Some(noun) },
                    prep: Some(words[prep_pos].to_owned()),
                    obj: if obj.is_empty() { None } else { Some(obj) },
                }
            }
        } else if words.len() > 1 {
            Self {
                verb: Some(words[0].to_owned()),
                noun: Some(words[1..].join(" ")),
                prep: None,
                obj: None,
            }
        } else {
            Self {
                verb: words.pop(),
                noun: None,
                prep: None,
                obj: None,
            }
        }
    }

    pub(crate) fn noun(&self) -> Option<&str> {
        self.noun.as_deref()
    }

    pub(crate) fn obj(&self) -> Option<&str> {
        self.obj.as_deref()
    }

    pub(crate) fn prep(&self) -> Option<&str> {
        self.prep.as_deref()
    }

    pub(crate) fn verb(&self) -> Option<&str> {
        self.verb.as_deref()
    }

    pub(crate) fn with_verb(mut self, verb: &str) -> Self {
        self.verb = Some(verb.to_owned());
        self
    }
}

fn alias(s: String) -> String {
    match s.as_str() {
        "n" => "north".to_owned(),
        "s" => "south".to_owned(),
        "e" => "east".to_owned(),
        "w" => "west".to_owned(),
        "ne" => "northeast".to_owned(),
        "nw" => "northwest".to_owned(),
        "se" => "southeast".to_owned(),
        "sw" => "southwest".to_owned(),
        "u" => "up".to_owned(),
        "d" => "down".to_owned(),
        _ => s,
    }
}
