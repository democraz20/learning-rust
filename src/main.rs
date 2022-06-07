// use std::io;
// use std::io::Write;
// use std::env;
use std::fs;
// mod num;
// use num::is_real_num;
// use core::ptr::null;
// use std::io::ErrorKind;

fn main() {
    let fileName = "file.txt";
    println!("in file : {}", fileName);
    
    let read = fs::read_to_string(fileName)
        .expect("Something went wrong reading the file");
    println!("text : \n{}", read.trim());
}
