#[cfg(not(target_arch = "wasm32"))]
use std::io;

pub use game::Game;

mod action;
mod container;
mod direction;
mod game;
mod item;
mod tokens;

pub(crate) fn list_names(names: &[&str], sep: &str) -> String {
    let a = if sep == "or" { "the" } else { "a" };

    match names.len() {
        0 => String::new(),
        1 => format!("{a} {}", names[0]),
        2 => format!("{a} {} {sep} {a} {}", names[0], names[1]),
        _ => {
            format!(
                "{a} {}, {sep} {a} {}",
                names[1..names.len() - 1]
                    .iter()
                    .fold(names[0].to_owned(), |acc, i| format!("{}, {a} {}", acc, i)),
                names[names.len() - 1]
            )
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
