use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Read;
// use std::io::Write;
// use std::fs;


fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("file.txt");
    
    dbg!(file);
    println!("OpenOptions");
    let mut file = File::create("file.txt").unwrap();
    dbg!(&file);

    println!("File::create");
    
    let mut file_contents = String::new();
    
    println!("file.write_all");
    
    file.write_all("test message waow2".as_bytes())
        .expect("unable to write file");
    
    println!("file.read_to_string");
    let mut file = File::open("file.txt").unwrap();
    file.read_to_string(&mut file_contents).expect("unable to read file");

    println!("current content: \n{}", file_contents);
}
