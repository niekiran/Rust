
use std::io::Write;

fn main() {
    print!("Hello, ");
	std::io::stdout().flush().unwrap();
    eprintln!("An error occurred: invalid input");
    let name = "John";
    let age = 30;
    let message = format!("My name is {} and I am {} years old",name,age);
    println!("{}",message);
    println!("Hello, World!");

}