use serde::{Deserialize, Serialize};

use crate::direction::Direction;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum Command {
    Again,
    Attack(String, String),
    Break(String),
    Burn(String, String),
    Clarify(String),
    Climb,
    Close(String),
    Drop(String),
    Eat(String),
    Examine(String),
    Hello,
    Help,
    Inventory,
    Look,
    Move(String),
    NoVerb,
    Open(String),
    Put(String, String),
    Sleep,
    Take(String),
    Unknown(String),
    Walk(String),
    Wear(String),
    Where(String),
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
            command: Self::parse(verb, &noun, &mut prep, &obj),
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

    fn parse(verb: &str, noun: &str, prep: &mut String, obj: &str) -> Command {
        match verb {
            _ if verb.is_direction() => Command::Walk(verb.to_string()),
            "again" | "g" => Command::Again,
            "attack" | "cut" | "hit" | "hurt" | "kill" => {
                Command::Attack("".to_owned(), "".to_owned())
            }
            "break" | "destroy" => Command::Break("".to_owned()),
            "burn" => Command::Burn("".to_owned(), "".to_owned()),
            "climb" => Command::Climb,
            "close" | "shut" => {
                if noun.is_empty() {
                    Command::Clarify(verb.to_string())
                } else {
                    Command::Close(noun.to_string())
                }
            }
            "drop" | "place" | "throw" => {
                if noun.is_empty() {
                    Command::Clarify(verb.to_string())
                } else if obj.is_empty() {
                    Command::Drop(noun.to_string())
                } else {
                    Command::Put(noun.to_string(), obj.to_string())
                }
            }
            "eat" | "consume" | "drink" | "quaff" => {
                if noun.is_empty() {
                    Command::Clarify(verb.to_string())
                } else {
                    Command::Eat(noun.to_string())
                }
            }
            "enter" => {
                if noun.is_empty() {
                    Command::Walk(verb.to_string())
                } else {
                    Command::Walk(noun.to_string())
                }
            }
            "examine" | "inspect" | "read" | "what" => {
                if noun.is_empty() {
                    Command::Clarify(verb.to_string())
                } else {
                    Command::Examine(noun.to_string())
                }
            }
            "hello" | "hi" => Command::Hello,
            "help" => Command::Help,
            "inventory" | "i" => Command::Inventory,
            "go" | "walk" => {
                if noun.is_empty() {
                    // TODO: "Where do you want to {}?"
                    Command::Clarify(verb.to_string())
                } else {
                    Command::Walk(noun.to_string())
                }
            }
            "look" | "l" => {
                if noun.is_empty() {
                    Command::Look
                } else {
                    Command::Examine(noun.to_string())
                }
            }
            "move" | "pull" | "push" => Command::Move("".to_owned()),
            "open" => {
                if noun.is_empty() {
                    Command::Clarify(verb.to_string())
                } else {
                    Command::Open(noun.to_string())
                }
            }
            "put" => {
                if prep == "on" {
                    if obj.is_empty() {
                        Command::Clarify("put on".to_owned())
                    } else {
                        Command::Wear(obj.to_string())
                    }
                } else if noun.is_empty() {
                    Command::Clarify(verb.to_string())
                } else if obj.is_empty() {
                    if prep.is_empty() {
                        prep.push_str("in");
                    }
                    Command::Clarify(format!("{} the {} {}", verb, noun, prep))
                } else {
                    Command::Put(noun.to_string(), obj.to_string())
                }
            }
            "take" | "hold" | "get" | "pick" | "remove" => {
                if noun.is_empty() {
                    Command::Clarify(verb.to_string())
                } else {
                    Command::Take(noun.to_string())
                }
            }
            "wait" | "z" | "sleep" => Command::Sleep,
            "where" | "find" | "see" => {
                if noun.is_empty() {
                    Command::NoVerb
                } else {
                    Command::Where(noun.to_string())
                }
            }
            "" => Command::NoVerb,
            _ => Command::Unknown(verb.to_string()),
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
            command: Self::parse(&verb, &noun, &mut prep, &obj),
            verb,
            noun,
            prep,
            obj,
        }
    }
}
