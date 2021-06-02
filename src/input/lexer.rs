use rayon::prelude::*;

use crate::input::CmdTokens;

#[derive(Clone, Debug)]
pub struct Lexer;

impl Lexer {
    pub fn lex(s: &str) -> CmdTokens {
        let words = Lexer::mod_words(Lexer::filter_parts(s));

        if words.is_empty() {
            CmdTokens::default()
        } else if words.len() < 2 {
            CmdTokens::new(words[0].to_owned())
        } else {
            let prep_pos = if cfg!(target_arch = "wasm32") {
                words
                    .iter()
                    .position(|r| ["in", "inside", "from", "on", "with"].contains(&r.as_str()))
            } else {
                words
                    .par_iter()
                    .position_any(|r| ["in", "inside", "from", "on", "with"].contains(&r.as_str()))
            };

            if let Some(pos) = prep_pos {
                if pos == 0 {
                    CmdTokens::new(words[0].to_owned()).with_obj(words[1..].join(" "))
                } else {
                    let obj = if words[1..pos].is_empty() {
                        None
                    } else {
                        Some(words[1..pos].join(" "))
                    };

                    let obj_prep = if words[pos + 1..].is_empty() {
                        None
                    } else {
                        Some(words[pos + 1..].join(" "))
                    };

                    CmdTokens::from(
                        Some(words[0].to_owned()),
                        obj,
                        Some(words[pos].to_owned()),
                        obj_prep,
                    )
                }
            } else {
                CmdTokens::new(words[0].to_owned()).with_obj(words[1..].join(" "))
            }
        }
    }

    fn filter_parts(s: &str) -> Vec<String> {
        if cfg!(target_arch = "wasm32") {
            s.split_whitespace()
                .map(str::to_lowercase)
                .filter(|w| {
                    !([
                        "a", "an", "around", "at", "of", "my", "that", "the", "through", "to", "'",
                    ])
                    .contains(&w.as_str())
                })
                .collect()
        } else {
            s.par_split_whitespace()
                .map(str::to_lowercase)
                .filter(|w| {
                    !([
                        "a", "an", "around", "at", "of", "my", "that", "the", "through", "to", "'",
                    ])
                    .contains(&w.as_str())
                })
                .collect()
        }
    }

    fn mod_words(mut words: Vec<String>) -> Vec<String> {
        for w in words.iter_mut() {
            match w.as_str() {
                "n" => *w = String::from("north"),
                "s" => *w = String::from("south"),
                "e" => *w = String::from("east"),
                "w" => *w = String::from("west"),
                "ne" => *w = String::from("northeast"),
                "nw" => *w = String::from("northwest"),
                "se" => *w = String::from("southeast"),
                "sw" => *w = String::from("southwest"),
                "u" => *w = String::from("up"),
                "d" => *w = String::from("down"),
                "r" => *w = String::from("again"),
                _ => (),
            }
        }
        words
    }
}
