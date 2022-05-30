use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    print!("Enter a number: ");

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input).expect("Failed to read line.");
    
    let intput=input.trim();
    match intput.parse::<u32>() {
        Ok(num) => println!("You entered: {num}"),
        Err(..) => println!("Not an integer: {intput}"),
    }
}


