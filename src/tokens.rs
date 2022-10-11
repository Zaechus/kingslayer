use serde::{Deserialize, Serialize};

use crate::direction::Direction;

const USELESS_WORDS: [&str; 15] = [
    "a", "am", "an", "across", "around", "at", "for", "is", "of", "my", "that", "the", "this",
    "through", "to",
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub(crate) enum Command {
    Again,
    Attack,
    Break,
    Burn,
    Climb,
    Close(String),
    Drop(String),
    Put(String, String),
    Eat,
    Hello,
    Help,
    Inventory,
    Look,
    Move,
    NoVerb,
    Open(String),
    Sleep,
    Take(String),
    Unknown(String),
    Walk(String),
    Wear(String),
    Where(String),
    Clarify(String),
    Examine(String),
}

impl Default for Command {
    fn default() -> Self {
        Self::Look
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub(crate) struct Tokens {
    command: Command,
    verb: String,
    noun: String,
    prep: String,
    obj: String,
}

impl Tokens {
    pub(crate) fn command(&self) -> &Command {
        &self.command
    }

    pub(crate) fn new(words: &[String]) -> Self {
        let words: Vec<_> = words
            .iter()
            .map(alias)
            .filter(|w| !USELESS_WORDS.contains(w))
            .collect();

        let mut verb = String::new();
        let mut noun = String::new();
        let mut prep = String::new();
        let mut obj = String::new();
        let command = if let Some(v) = words.first() {
            verb = v.to_string();

            if let Some(prep_pos) = words.iter().position(|w| PREPOSITIONS.contains(w)) {
                if prep_pos == 0 {
                    prep = words[0].to_owned();
                    obj = words[1..].join(" ");
                } else {
                    noun = words[1..prep_pos].join(" ");
                    prep = words[prep_pos].to_owned();
                    obj = words[prep_pos + 1..].join(" ");
                }
            } else {
                noun = words[1..].join(" ");
            }

            match *v {
                _ if words[0].is_direction() => Command::Walk(v.to_string()),
                "again" | "g" => Command::Again,
                "attack" | "cut" | "hit" | "hurt" | "kill" => Command::Attack,
                "break" | "destroy" => Command::Break,
                "burn" => Command::Burn,
                "climb" => Command::Climb,
                "close" | "shut" => {
                    if noun.is_empty() {
                        Command::Clarify(v.to_string())
                    } else {
                        Command::Close(noun.clone())
                    }
                }
                // TODO: use code in put
                "drop" | "place" | "throw" => {
                    if noun.is_empty() {
                        Command::Clarify(v.to_string())
                    } else if obj.is_empty() {
                        Command::Drop(noun.clone())
                    } else {
                        Command::Put(noun.clone(), obj.clone())
                    }
                }
                "eat" | "consume" | "drink" | "quaff" => Command::Eat,
                "examine" | "inspect" | "read" | "what" => {
                    if noun.is_empty() {
                        Command::Clarify(v.to_string())
                    } else {
                        Command::Examine(noun.clone())
                    }
                }
                "hello" | "hi" => Command::Hello,
                "help" => Command::Help,
                "inventory" | "i" => Command::Inventory,
                "go" | "enter" | "walk" => {
                    if noun.is_empty() {
                        Command::Clarify(v.to_string())
                    } else {
                        Command::Walk(noun.clone())
                    }
                }
                "look" | "l" => {
                    if noun.is_empty() {
                        Command::Look
                    } else {
                        Command::Examine(noun.clone())
                    }
                }
                "move" | "pull" | "push" => Command::Move,
                "open" => {
                    if noun.is_empty() {
                        Command::Clarify(v.to_string())
                    } else {
                        Command::Open(noun.clone())
                    }
                }
                "put" => {
                    if prep == "on" {
                        Command::Wear(obj.clone())
                    } else if noun.is_empty() {
                        Command::Clarify(v.to_string())
                    } else if obj.is_empty() {
                        if prep.is_empty() {
                            Command::Clarify(format!("{} the {} in", verb, noun))
                        } else {
                            Command::Clarify(format!("{} the {} {}", verb, noun, prep))
                        }
                    } else {
                        Command::Put(noun.clone(), obj.clone())
                    }
                }
                "take" | "hold" | "get" | "pick" | "remove" => {
                    if noun.is_empty() {
                        Command::Clarify(v.to_string())
                    } else {
                        Command::Take(noun.clone())
                    }
                }
                "wait" | "z" | "sleep" => Command::Sleep,
                "where" | "find" | "see" => {
                    if noun.is_empty() {
                        Command::NoVerb
                    } else {
                        Command::Where(noun.clone())
                    }
                }
                _ => Command::Unknown(v.to_string()),
            }
        } else {
            Command::NoVerb
        };

        Self {
            command,
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

    pub(crate) fn prep(&self) -> &str {
        self.prep.as_ref()
    }

    pub(crate) fn set_noun(&mut self, noun: String) {
        self.noun = noun;
    }

    pub(crate) fn verb(&self) -> &str {
        self.verb.as_ref()
    }
}
