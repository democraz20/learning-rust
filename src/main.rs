use std::io;
use std::io::Write;
// use core::ptr::null;

mod utils;
use utils::*;
extern crate crossterm;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::io::stdout;
extern crate term_size;

fn main() -> Result<()> {
    //initializing alternate screen
    execute!(stdout(), EnterAlternateScreen)?;
    //EVERYTHING AFTER THIS IS IN THE ALTERNATESCREEN
    //set cursor to 0,0
    execute!(stdout(), MoveTo(0, 0),);
    loop {
        //normal console IO
        let mut input = String::new();
        print!(">>>");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        //properly getting input string for commands
        //this way, possible args in the future will be lowercase
        //need fix, probably with an array or vec
        //then have the first index as the command name
        let input = input.trim().to_lowercase();
        //commands
        if input == "exit" {
            //leaving alternate screen when 
            //"exit" is executed
            execute!(stdout(), LeaveAlternateScreen)?;
            break;
        } else if input == "hello" {
            println!("world!");
        } else if input == "" {
            continue;
        }
        else if input == "clear" {
            clear_screen();
        }
        else if input == "alert_screen" {
            alert_screen();
        }
        else {
            println!("unrecognized input : {}", input);
        }
    }
    //return result type
    Ok(())
}
