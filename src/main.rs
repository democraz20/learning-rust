use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    print!("Enter a number: ");

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input).expect("Failed to read line.");
    let input=input.trim();
    let mut gotint = parsetou32(input);
    println!("You entered: {gotint}");
    gotint = gotint+5;
    println!("Your number plus 5 is: {gotint}");
}


fn parsetou32(input: &str) -> u32 {
    let out: u32 = input.parse::<u32>().unwrap();
    return out;
}

