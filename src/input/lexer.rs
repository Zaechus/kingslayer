use rayon::prelude::*;

use crate::input::CmdTokens;

#[derive(Clone, Debug)]
pub struct Lexer;

impl Lexer {
    pub fn lex(s: &str) -> CmdTokens {
        let words = Lexer::mod_words(&Lexer::filter_parts(s));

        if words.is_empty() {
            CmdTokens::new(0, None, None, None, None)
        } else if words.len() < 2 {
            CmdTokens::new(1, Some(words[0].to_owned()), None, None, None)
        } else if let Some(pos) = words
            .par_iter()
            .position_any(|r| ["in", "inside", "from", "on", "with"].contains(&r.as_str()))
        {
            CmdTokens::new(
                words.len(),
                Some(words[0].to_owned()),
                Some(words[1..pos].join(" ")),
                Some(words[pos].to_owned()),
                Some(words[pos + 1..].join(" ")),
            )
        } else {
            CmdTokens::new(
                words.len(),
                Some(words[0].to_owned()),
                Some(words[1..].join(" ")),
                None,
                None,
            )
        }
    }

    fn filter_parts(s: &str) -> Vec<String> {
        let words: Vec<String> = s
            .par_split_whitespace()
            .map(str::to_lowercase)
            .filter(|w| {
                !([
                    "a", "an", "around", "at", "of", "my", "that", "the", "through", "to", "'",
                ])
                .contains(&w.as_str())
            })
            .collect();
        words
    }

    fn mod_words(words: &[String]) -> Vec<String> {
        let mut modified = Vec::with_capacity(5 * words.len());
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
        modified.shrink_to_fit();
        modified
    }
}
