use std::io;
use std::io::Write;
// use core::ptr::null;
// use std::io::ErrorKind;

fn main() {
    loop {
        print!("Enter a number: ");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input = input.trim();
        // let gotint = parsetou32(input);
        let test = test(input);
        println!("You entered: {test}");
    }
}

fn parsetou32(input: &str) -> u32 {
    match input.parse::<u32>() {
        Ok(i) => i,
        Err(_) => {
            println!("Not a number!");
            0
        },
    }
}

fn test(input: &str) -> &str {
    let x = input;
    match input.parse::<u32>(){
        Ok(_i) => return x,
        Err(_) => return "Not a number!",
    }
}
