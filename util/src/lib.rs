use std::io;

pub fn get_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Could not read from Stdin");
    return buffer;
}
