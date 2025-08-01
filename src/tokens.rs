use serde::{Deserialize, Serialize};

use crate::{action::Action, direction::Direction};

const USELESS_WORDS: [&str; 17] = [
    "a", "am", "an", "across", "around", "at", "for", "is", "of", "my", "no", "that", "the",
    "this", "through", "to", "yes",
];
const PREPOSITIONS: [&str; 6] = ["in", "from", "on", "out", "under", "with"];

fn alias(s: &str) -> &str {
    match s {
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
        "inside" => "in",
        "outside" => "out",
        "everything" => "all",
        "them" => "it",
        _ => s,
    }
}

macro_rules! do_or_ask {
    ($action:ident, $noun:ident, $verb:ident) => {
        if $noun.is_empty() {
            Action::what_do($verb)
        } else {
            Action::$action($noun.to_owned())
        }
    };
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
            .map(|s| alias(s))
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
            verb.to_string()
        } else {
            String::new()
        };

        Self {
            action: Self::parse(&verb, &noun, &mut prep, &obj),
            verb,
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
            _ if verb.is_direction() => Action::Walk(verb.to_owned()),
            "again" | "g" => Action::Again,
            "attack" | "cut" | "hit" | "hurt" | "kill" | "murder" | "slash" | "slay" | "stab" => {
                if prep.is_empty() {
                    prep.push_str("with");
                }
                match (noun.is_empty(), prep.as_str(), obj.is_empty()) {
                    (false, _, false) => Action::Attack(noun.to_owned(), obj.to_owned()),
                    (true, _, false) => Action::what_do(&format!("{verb} {prep} the {obj}")),
                    (false, _, true) => Action::what_do(&format!("{verb} the {noun} {prep}")),
                    (true, _, true) => Action::what_do(verb),
                }
            }
            "break" | "destroy" | "smash" => Action::Break(String::new()),
            "climb" => Action::Climb,
            "close" | "shut" => do_or_ask!(Close, noun, verb),
            "drop" | "throw" => {
                if noun.is_empty() {
                    Action::what_do(verb)
                } else if obj.is_empty() {
                    Action::Drop(noun.to_owned())
                } else {
                    Action::Put(noun.to_owned(), obj.to_owned())
                }
            }
            "eat" | "consume" | "devour" | "drink" | "quaff" => do_or_ask!(Eat, noun, verb),
            "enter" => {
                if noun.is_empty() {
                    Action::Walk(verb.to_owned())
                } else {
                    Action::Walk(noun.to_owned())
                }
            }
            "examine" | "inspect" | "read" | "what" | "x" => do_or_ask!(Examine, noun, verb),
            "go" | "walk" => {
                if noun.is_empty() {
                    match prep.as_str() {
                        "in" => Action::Walk("enter".to_owned()),
                        "out" => Action::Walk("exit".to_owned()),
                        _ => Action::Clarify(format!("Where do you want to {verb}?")),
                    }
                } else {
                    Action::Walk(noun.to_owned())
                }
            }
            "give" => todo!(),
            "hello" | "hi" => Action::Hello,
            "help" => Action::Help,
            "in" => Action::Walk("enter".to_owned()),
            "inventory" | "i" => Action::Inventory,
            "light" => todo!(),
            "look" | "l" => {
                if noun.is_empty() {
                    Action::Look
                } else {
                    Action::Examine(noun.to_owned())
                }
            }
            "move" | "pull" | "push" => {
                if noun.is_empty() {
                    Action::what_do(verb)
                } else {
                    Action::Move(noun.to_owned())
                }
            }
            "open" => do_or_ask!(Open, noun, verb),
            "out" => Action::Walk("exit".to_owned()),
            "pick" => match noun {
                "" => Action::what_do("pick"),
                noun if noun.starts_with("up ") => Action::Take(noun[2..].to_string()),
                _ => Action::Take(noun.to_owned()),
            },
            "put" | "place" => {
                if prep.is_empty() {
                    prep.push_str("in");
                }
                match (noun.is_empty(), prep.as_str(), obj.is_empty()) {
                    (false, "on", true) => Action::Wear(noun.to_owned()),
                    (true, "on", false) => Action::Wear(obj.to_owned()),
                    (true, "on", true) => Action::what_do("put on"),
                    (false, _, false) => Action::Put(noun.to_owned(), obj.to_owned()),
                    (true, _, false) => Action::what_do(&format!("{verb} {prep} the {obj}")),
                    (false, _, true) => Action::what_do(&format!("{verb} the {noun} {prep}")),
                    (true, _, true) => Action::what_do(verb),
                }
            }
            "take" | "get" | "grab" | "hold" | "remove" => do_or_ask!(Take, noun, verb),
            "version" => Action::Version,
            "wait" | "z" | "sleep" => Action::Sleep,
            "wear" | "don" => Action::Wear(noun.to_owned()),
            "where" | "find" | "see" => {
                if noun.is_empty() {
                    Action::NoVerb
                } else {
                    Action::Where(noun.to_owned())
                }
            }
            "" => Action::NoVerb,
            _ => Action::Unknown(verb.to_owned()),
        }
    }

    pub(crate) fn prep(&self) -> &str {
        self.prep.as_ref()
    }

    pub(crate) fn verb(&self) -> &str {
        self.verb.as_ref()
    }

    pub(crate) fn with(verb: String, noun: String, mut prep: String, obj: String) -> Self {
        let noun = noun
            .split_whitespace()
            .map(alias)
            .filter(|w| !USELESS_WORDS.contains(w))
            .collect::<Vec<_>>()
            .join(" ");
        let obj = obj
            .split_whitespace()
            .map(alias)
            .filter(|w| !USELESS_WORDS.contains(w))
            .collect::<Vec<_>>()
            .join(" ");

        Self {
            action: Self::parse(&verb, &noun, &mut prep, &obj),
            verb,
            noun,
            prep,
            obj,
        }
    }
}
