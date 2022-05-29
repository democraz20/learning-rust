use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    print!("Enter your name :");
    
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("unable to read line");
    println!("Hello, {input}!", input = input.trim()); // trim removes the newline character
}
