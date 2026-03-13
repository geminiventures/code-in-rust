use std::io;

fn main() {
    // Write your code below
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input == "1" {println!("T")}
    else {println!("F")}

}