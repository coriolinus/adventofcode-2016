extern crate day05lib;
use day05lib::get_password;

fn main() {
    const INPUT: &'static str = "ugkcyxxp";
    println!("Hashing...");
    let password = get_password(INPUT).into_iter().cloned().collect::<String>();
    println!("{}", password);
}
