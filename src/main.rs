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
        let vec2: Vec<&str> = vec.clone();
        let mut first_num = String::new();
        let mut second_num = String::new();
        let mut operator = String::new();
        let mut index = 0;
        for i in vec {
            if is_real_num(i) {
                // let s = i.to_string();
                first_num = first_num + i;
                // println!("{} is a real number.", i);
            } else {
                if i == "+" || i == "-" || i == "*" || i == "/" {
                    operator = i.to_string();
                    // println!("{} is an operator.", operator);
                    index = vec2.iter().position(|&r| r == i).unwrap();
                    // println!("index : {}", index);
                    break;
                }
                else if i == "."{
                    first_num = first_num + i;
                } 
                else {
                    if i == " "{
                        continue;
                    }else{
                        println!("{} is not a real number.", i);
                    }
                }
            }  
        }
        let secnumvec: Vec<&str> = (&vec2[index + 1..]).to_vec();
        for i in secnumvec{
            second_num = second_num + i;
        }
        let first_num = match first_num.parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        let second_num = match second_num.parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        let mut res;
        if operator == "+" {
            res = first_num + second_num;
        } else if operator == "-" {
            res = first_num - second_num;
        } else if operator == "*" {
            res = first_num * second_num;
        } else if operator == "/" {
            res = first_num / second_num;
        } else {
            println!("{} is not a valid operator.", operator);
            continue;
        }
        println!("Result : {}", res);
    }
}
