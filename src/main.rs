use std::{env, error, fs, process::ExitCode};

use kingslayer::*;

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, Box<dyn error::Error>> {
    let mut game: Game = if let Some(filename) = env::args().nth(1) {
        fs::read_to_string(filename)?.parse()?
    } else {
        include_str!("world.ron").parse()?
    };

    game.play()?;

    Ok(ExitCode::SUCCESS)
}
