use rayon::prelude::*;

use crate::tokens::Tokens;

const USELESS_WORDS: [&str; 10] = [
    "a", "an", "around", "at", "of", "my", "that", "the", "through", "to",
];
const PREPOSITIONS: [&str; 5] = ["in", "inside", "from", "on", "with"];

pub(crate) fn lex(input: String) -> Tokens {
    let mut words: Vec<_> = input
        .par_split_whitespace()
        .map(str::to_lowercase)
        .filter(|w| !USELESS_WORDS.contains(&w.as_str()))
        .map(alias)
        .collect();

    if let Some(prep_pos) = words
        .par_iter()
        .position_any(|w| PREPOSITIONS.contains(&w.as_str()))
    {
        if prep_pos == 0 {
            Tokens::new(
                None,
                None,
                Some(words[0].to_owned()),
                Some(words[1..].join(" ")),
            )
        } else {
            Tokens::new(
                Some(words[0].to_owned()),
                Some(words[1..prep_pos].join(" ")),
                Some(words[prep_pos].to_owned()),
                Some(words[prep_pos + 1..].join(" ")),
            )
        }
    } else if words.len() > 1 {
        Tokens::new(
            Some(words[0].to_owned()),
            Some(words[1..].join(" ")),
            None,
            None,
        )
    } else {
        Tokens::new(words.pop(), None, None, None)
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
