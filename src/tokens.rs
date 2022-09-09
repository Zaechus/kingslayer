use rayon::prelude::*;

const USELESS_WORDS: [&str; 10] = [
    "a", "an", "around", "at", "of", "my", "that", "the", "through", "to",
];
const PREPOSITIONS: [&str; 5] = ["in", "inside", "from", "on", "with"];

#[derive(Debug)]
pub(crate) struct Tokens {
    verb: Option<String>,
    obj: Option<String>,
    prep: Option<String>,
    obj_prep: Option<String>,
}

impl Tokens {
    pub(crate) fn new(input: String) -> Self {
        let mut words: Vec<_> = input
            .par_split_whitespace()
            .map(str::to_lowercase)
            .map(alias)
            .filter(|w| !USELESS_WORDS.contains(&w.as_str()))
            .collect();

        if let Some(prep_pos) = words
            .par_iter()
            .position_any(|w| PREPOSITIONS.contains(&w.as_str()))
        {
            if prep_pos == 0 {
                Self {
                    verb: None,
                    obj: None,
                    prep: Some(words[0].to_owned()),
                    obj_prep: Some(words[1..].join(" ")),
                }
            } else {
                let obj = words[1..prep_pos].join(" ");
                let obj_prep = words[prep_pos + 1..].join(" ");

                Self {
                    verb: Some(words[0].to_owned()),
                    obj: if obj.is_empty() { None } else { Some(obj) },
                    prep: Some(words[prep_pos].to_owned()),
                    obj_prep: if obj_prep.is_empty() {
                        None
                    } else {
                        Some(obj_prep)
                    },
                }
            }
        } else if words.len() > 1 {
            Self {
                verb: Some(words[0].to_owned()),
                obj: Some(words[1..].join(" ")),
                prep: None,
                obj_prep: None,
            }
        } else {
            Self {
                verb: words.pop(),
                obj: None,
                prep: None,
                obj_prep: None,
            }
        }
    }
}

impl Tokens {
    pub(crate) fn verb(&self) -> Option<&str> {
        self.verb.as_deref()
    }

    pub(crate) fn obj(&self) -> Option<&str> {
        self.obj.as_deref()
    }

    // pub(crate) fn prep(&self) -> Option<&str> {
    //     self.prep.as_deref()
    // }

    // pub(crate) fn obj_prep(&self) -> Option<&str> {
    //     self.obj_prep.as_deref()
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
