
use std::io;

fn main() {
    let mut age_input = String::new();
    let mut height_input = String::new();
    let mut adult_input = String::new();

    io::stdin().read_line(&mut age_input).unwrap();
    io::stdin().read_line(&mut height_input).unwrap();
    io::stdin().read_line(&mut adult_input).unwrap();

    let age: i32 = age_input.trim().parse().unwrap();
    let height: i32 = height_input.trim().parse().unwrap();
    let has_adult: bool = adult_input.trim().parse().unwrap();

    if age < 12 {
        println!("Sorry, you're too young");
    } else if height < 150 {
        println!("Sorry, you're not tall enough");
    } else if age < 15 && !has_adult {
        println!("Sorry, you need an adult with you");
    } else if age < 15 && has_adult {
        println!("You can ride with adult supervision!");
    } else {
        println!("You can ride by yourself!");
    }



}