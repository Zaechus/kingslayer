use std::io;

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading");
    input.trim().to_owned()
}
