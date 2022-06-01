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
        
        let convert: i32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            },
        };
        // let gotint = parsetou32(input);
        println!("You entered: {convert}");
    }
}

