use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut num1: f64 = 0.0;
    let mut num2: f64 = 0.0;
    io::stdin().read_line(&mut input1).unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    num1 = input1.trim().parse::<f64>().unwrap();
    num2 = input2.trim().parse::<f64>().unwrap();
    let mut sum = num1 + num2;
    let mut diff = num1 - num2;
    let mut prod = num1 * num2;
    let mut quot = num1 / num2;
    println!("Calculator App");
    println!("Sum: {}", sum);
    println!("Difference: {}", diff);
    println!("Product: {}", prod);
    println!("Quotient: {}", quot);
}