const USELESS_WORDS: [&str; 11] = [
    "a", "an", "around", "at", "for", "of", "my", "that", "the", "through", "to",
];
const PREPOSITIONS: [&str; 5] = ["in", "inside", "from", "on", "with"];

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
}

impl Tokens {
    pub(crate) fn verb(&self) -> Option<&str> {
        self.verb.as_deref()
    }

    pub(crate) fn noun(&self) -> Option<&str> {
        self.noun.as_deref()
    }

    // pub(crate) fn prep(&self) -> Option<&str> {
    //     self.prep.as_deref()
    // }

    // pub(crate) fn obj(&self) -> Option<&str> {
    //     self.obj.as_deref()
    // }
}

fn alias(s: String) -> String {
    match s.as_str() {
        "north" => "n".to_owned(),
        "south" => "s".to_owned(),
        "east" => "e".to_owned(),
        "west" => "w".to_owned(),
        "northeast" => "ne".to_owned(),
        "northwest" => "nw".to_owned(),
        "southeast" => "se".to_owned(),
        "southwest" => "sw".to_owned(),
        "up" => "u".to_owned(),
        "down" => "d".to_owned(),
        _ => s,
    }
}
