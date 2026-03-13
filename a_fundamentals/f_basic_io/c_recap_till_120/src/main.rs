use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let age: u32 = input.trim().parse().unwrap();
    println!("{} years till 120", 120 - age);
}
