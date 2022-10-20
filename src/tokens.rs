use serde::{Deserialize, Serialize};

use crate::{action::Action, direction::Direction};

const USELESS_WORDS: [&str; 17] = [
    "a", "am", "an", "across", "around", "at", "for", "is", "of", "my", "no", "that", "the",
    "this", "through", "to", "yes",
];
const PREPOSITIONS: [&str; 6] = ["in", "inside", "from", "on", "under", "with"];

fn alias(s: &String) -> &str {
    match s.as_str() {
        "n" => "north",
        "s" => "south",
        "e" => "east",
        "w" => "west",
        "ne" => "northeast",
        "nw" => "northwest",
        "se" => "southeast",
        "sw" => "southwest",
        "u" => "up",
        "d" => "down",
        _ => s,
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub(crate) struct Tokens {
    action: Action,
    verb: String,
    noun: String,
    prep: String,
    obj: String,
}

impl Tokens {
    pub(crate) fn action(&self) -> &Action {
        &self.action
    }

    pub(crate) fn new(words: &[String]) -> Self {
        let words: Vec<_> = words
            .iter()
            .map(alias)
            .filter(|w| !USELESS_WORDS.contains(w))
            .collect();

        let mut noun = String::new();
        let mut prep = String::new();
        let mut obj = String::new();
        let verb = if let Some(verb) = words.first() {
            if let Some(prep_pos) = words.iter().position(|w| PREPOSITIONS.contains(w)) {
                if prep_pos != 0 {
                    noun = words[1..prep_pos].join(" ");
                }
                prep = words[prep_pos].to_owned();
                obj = words[prep_pos + 1..].join(" ");
            } else {
                noun = words[1..].join(" ");
            }
            verb
        } else {
            ""
        };

        Self {
            action: Self::parse(verb, &noun, &mut prep, &obj),
            verb: verb.to_string(),
            noun,
            prep,
            obj,
        }
    }

    pub(crate) fn noun(&self) -> &str {
        self.noun.as_ref()
    }

    pub(crate) fn obj(&self) -> &str {
        self.obj.as_ref()
    }

    fn parse(verb: &str, noun: &str, prep: &mut String, obj: &str) -> Action {
        match verb {
            _ if verb.is_direction() => Action::Walk(verb.to_string()),
            "again" | "g" => Action::Again,
            "attack" | "cut" | "hit" | "hurt" | "kill" => {
                Action::Attack("".to_owned(), "".to_owned())
            }
            "break" | "destroy" => Action::Break("".to_owned()),
            "burn" => Action::Burn("".to_owned(), "".to_owned()),
            "climb" => Action::Climb,
            "close" | "shut" => {
                if noun.is_empty() {
                    Action::what_do(verb)
                } else {
                    Action::Close(noun.to_string())
                }
            }
            "drop" | "throw" => {
                if noun.is_empty() {
                    Action::what_do(verb)
                } else if obj.is_empty() {
                    Action::Drop(noun.to_string())
                } else {
                    Action::Put(noun.to_string(), obj.to_string())
                }
            }
            "eat" | "consume" | "drink" | "quaff" => {
                if noun.is_empty() {
                    Action::what_do(verb)
                } else {
                    Action::Eat(noun.to_string())
                }
            }
            "enter" => {
                if noun.is_empty() {
                    Action::Walk(verb.to_string())
                } else {
                    Action::Walk(noun.to_string())
                }
            }
            "examine" | "inspect" | "read" | "what" => {
                if noun.is_empty() {
                    Action::what_do(verb)
                } else {
                    Action::Examine(noun.to_string())
                }
            }
            "hello" | "hi" => Action::Hello,
            "help" => Action::Help,
            "inventory" | "i" => Action::Inventory,
            "go" | "walk" => {
                if noun.is_empty() {
                    Action::Clarify(format!("Where do you want to {}?", verb))
                } else {
                    Action::Walk(noun.to_string())
                }
            }
            "look" | "l" => {
                if noun.is_empty() {
                    Action::Look
                } else {
                    Action::Examine(noun.to_string())
                }
            }
            "move" | "pull" | "push" => Action::Move("".to_owned()),
            "open" => {
                if noun.is_empty() {
                    Action::what_do(verb)
                } else {
                    Action::Open(noun.to_string())
                }
            }
            "put" | "place" => {
                if prep.is_empty() {
                    prep.push_str("in");
                }
                match (noun.is_empty(), prep.as_str(), obj.is_empty()) {
                    (false, "on", true) => Action::Wear(noun.to_owned()),
                    (true, "on", false) => Action::Wear(obj.to_owned()),
                    (true, "on", true) => Action::what_do("put on"),
                    (false, _, false) => Action::Put(noun.to_owned(), obj.to_owned()),
                    (true, _, false) => Action::what_do(&format!("{} {} the {}", verb, prep, obj)),
                    (false, _, true) => Action::what_do(&format!("{} the {} {}", verb, noun, prep)),
                    (true, _, true) => Action::what_do(verb),
                }
            }
            "take" | "hold" | "get" | "pick" | "remove" => {
                if noun.is_empty() {
                    Action::what_do(verb)
                } else {
                    Action::Take(noun.to_string())
                }
            }
            "wait" | "z" | "sleep" => Action::Sleep,
            "where" | "find" | "see" => {
                if noun.is_empty() {
                    Action::NoVerb
                } else {
                    Action::Where(noun.to_string())
                }
            }
            "" => Action::NoVerb,
            _ => Action::Unknown(verb.to_string()),
        }
    }

    pub(crate) fn prep(&self) -> &str {
        self.prep.as_ref()
    }

    pub(crate) fn verb(&self) -> &str {
        self.verb.as_ref()
    }

    pub(crate) fn with(verb: String, noun: String, mut prep: String, obj: String) -> Self {
        Self {
            action: Self::parse(&verb, &noun, &mut prep, &obj),
            verb,
            noun,
            prep,
            obj,
        }
    }
}
