use std::io;
use std::io::Write;
mod num;
use num::is_real_num;
// use core::ptr::null;
// use std::io::ErrorKind;

fn main() {
    loop {
        print!("Input : ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        // input = input.trim();
        let splitted = input.split("");
        let mut vec: Vec<&str> = splitted.collect();
        vec.pop();
        vec.pop();
        vec.remove(0);
        let mut first_num = String::new();
        for i in vec {
            if is_real_num(i) {
                // let s = i.to_string();
                first_num = first_num + i;
                println!("{} is a real number.", i);
            } else {
                if i == "+" || i == "-" || i == "*" || i == "/" {
                    let operator = i;
                    println!("{} is an operator.", operator);
                    break;
                } else {
                    if i == " "{
                        continue;
                    }else{
                        println!("{} is not a real number.", i);
                    }
                }
            }
        }
    }
}
