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
    println!("Calculator App");
    println!("{}", num1);
    println!("{}", num2);

}