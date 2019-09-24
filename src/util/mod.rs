use std::io;

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading stdin");
    input.trim().to_owned()
}

mod response;

pub use response::do_what;
pub use response::dont_have;
