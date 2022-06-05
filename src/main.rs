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
        let mut vec2: Vec<&str> = vec.clone();
        let mut first_num = String::new();
        let mut second_num = String::new();
        let mut operator = String::new();
        let mut index;
        for i in vec {
            if is_real_num(i) {
                // let s = i.to_string();
                first_num = first_num + i;
                println!("{} is a real number.", i);
            } else {
                if i == "+" || i == "-" || i == "*" || i == "/" {
                    operator = i.to_string();
                    println!("{} is an operator.", operator);
                    index = vec2.iter().position(|&r| r == i).unwrap();
                    println!("index : {}", index);
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
        for i in vec2 {
            if i == operator {
                println!("operator : {}", i);
            }
        }
    }
}
