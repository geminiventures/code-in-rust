fn main()
{
    // all 3 are valid ways to create a String
    let string1: String = "hello".to_string();
    let string2: String = String::from("hello");
    let string3 = "hello".to_owned();
    println!("{} {} {}", string1, string2, string3);
}
