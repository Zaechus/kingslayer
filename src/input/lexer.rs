use serde_derive::{Deserialize, Serialize};

use crate::types::CmdTokens;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lexer {
    filter_out: Vec<String>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            filter_out: vec!["a", "an", "at", "my", "of", "that", "the", "through", "to"]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
        }
    }

    pub fn lex(&self, s: &str) -> CmdTokens {
        let words = self.mod_words(&self.filter_parts(s));

        if words.len() < 2 {
            CmdTokens::new(1, &words[0], "", "", "")
        } else if let Some(pos) = words
            .iter()
            .position(|r| r == "in" || r == "inside" || r == "from" || r == "on" || r == "with")
        {
            CmdTokens::new(
                words.len(),
                &words[0],
                &words[1..pos].join(" "),
                &words[pos],
                &words[pos + 1..].join(" "),
            )
        } else {
            CmdTokens::new(words.len(), &words[0], &words[1..].join(" "), "", "")
        }
    }

    fn filter_parts(&self, s: &str) -> Vec<String> {
        let mut words: Vec<String> = s.split_whitespace().map(str::to_lowercase).collect();
        words.retain(|w| !(&self.filter_out).contains(&w));
        words
    }

    fn mod_words(&self, words: &[String]) -> Vec<String> {
        let mut modified = Vec::with_capacity(words.len());
        for w in words {
            modified.push(
                match w.as_str() {
                    "north" => "n",
                    "south" => "s",
                    "east" => "e",
                    "west" => "w",
                    "northeast" => "ne",
                    "northwest" => "nw",
                    "southeast" => "se",
                    "southwest" => "sw",
                    "up" => "u",
                    "down" => "d",
                    _ => w,
                }
                .to_owned(),
            );
        }
        modified
    }
}
