// Copyright (c) 2018 Maxwell Anderson

use std::io;

/// gets text input from the user and returns a String
pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading");
    input.trim().to_owned()
}
