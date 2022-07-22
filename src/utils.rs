use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use crossterm::{terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::execute;
// use crossterm::Result;

use crossterm::style::Stylize;

use std::io;
// use std::process;
use std::io::Write;
use std::io::stdout;
use std::time::Duration;

pub fn clear_screen_alternate() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //for use within alternate screen
}

#[allow(unused_must_use)]
pub fn text_input_raw() -> String{
    let mut input = String::new();
    // terminal::disable_raw_mode();
    match terminal::disable_raw_mode() {
        Ok(_) => {
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("unable to read line");  
            terminal::enable_raw_mode();
            input
        },
        Err(error) => panic!("unable to disable raw mode (in text_input_raw()) {}", error)
    }
    // input
    // Ok(())
    // String::from(input)
}

pub fn print_item(index: usize, items: &mut Vec<String>){
    println!();
    // let item = vec!["item_1", "item_2", "item_3", "item_4", "item_5"];
    for(ind, ele) in items.iter().enumerate() {
        // let ind = usize_to_u16(ind);
        if ind+1 == index {
            println!(" {} < \r", ele.clone().red());
        } else {
            println!(" {} \r", ele);
        }
        // print!(" ({}, {}) " , ele, ind)
    }
    println!("\r");
}


#[allow(dead_code)]
pub fn usize_to_u16(v: usize) -> u32 {
    v as u32
}