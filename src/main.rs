use std::io;
use std::io::Write;
// use core::ptr::null;
// use std::io::ErrorKind;

fn main() {
    println!("===[Calculator]===");
    loop {
        print!("Enter first number: ");

        let mut firstnum = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut firstnum)
            .expect("Failed to read line.");
        
        let firstnum: i32 = match firstnum.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            },
        };

        let mut operator = String::new();
        print!("Enter operator: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read line.");

        let mut secondnum = String::new();
        print!("Enter second number: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut secondnum)
            .expect("Failed to read line.");
        
        let secondnum: i32 = match secondnum.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            },
        };
        //types of operators
        if operator.trim() == "+" {
            println!("{} + {} = {}", firstnum, secondnum, firstnum + secondnum);
        } else if operator.trim() == "-" {
            println!("{} - {} = {}", firstnum, secondnum, firstnum - secondnum);
        } else if operator.trim() == "*" {
            println!("{} * {} = {}", firstnum, secondnum, firstnum * secondnum);
        } else if operator.trim() == "/" {
            println!("{} / {} = {}", firstnum, secondnum, firstnum / secondnum);
        } else {
            println!("Invalid operator!");
        }
        // let gotint = parsetou32(input);
    }
}

