use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use crossterm::{terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::execute;
// use crossterm::Result;

use crossterm::style::Stylize;

// use std::io;
// use std::process;
// use std::io::Write;
use std::io::stdout;
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

/*
TODO
put in alternate screen
actually doing the visuals
*/
#[allow(unused_must_use)]
fn main() -> crossterm::Result<()> {
    execute!(stdout(), EnterAlternateScreen);
    clear_screen_alternate();
    start()?;
    println!("Exiting Program \r");
    terminal::disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen);

    // process::exit(1);
    Ok(())
}

#[allow(unused_must_use)]
fn start() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    let mut index: u32 = 1;
    let mut items = vec!["item_1", "item_2", "item_3", "item_4", "item_5"];
    let index_limit = items.len(); let index_limit = index_limit as u32;
    print_item(index as usize, &mut items);
    println!("Recording Key Started"); 
    clear_screen_alternate();
    loop {    
        terminal::enable_raw_mode()?;
        loop {
            if event::poll(Duration::from_millis(1000))? {
                if let Event::Key(event) = event::read()? {
                    match event {
                        KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: event::KeyModifiers::CONTROL, /* modify */
                        } => {
                            // println!("Exiting Program \r");
                            // terminal::disable_raw_mode()?;
                            // execute!(stdout(), LeaveAlternateScreen);

                            // process::exit(1);
                            return Ok(());
                        },
                        KeyEvent {
                            code: KeyCode::Enter,
                            modifiers: event::KeyModifiers::NONE,
                        } => {},

                        //
                        KeyEvent {
                            code: KeyCode::Right,
                            modifiers: event::KeyModifiers::NONE
                        } => {  
                            if index < index_limit {
                                index += 1;
                            }
                        },
                        KeyEvent {
                            code: KeyCode::Left,
                            modifiers: event::KeyModifiers::NONE
                        } => { 
                            if index > 1 {
                                index -=1 ;
                            }
                        },

                        KeyEvent {
                            code: KeyCode::Down,
                            modifiers: event::KeyModifiers::NONE
                        } => {  
                            if index < index_limit {
                                index += 1;
                            }
                        },
                        KeyEvent {
                            code: KeyCode::Up,
                            modifiers: event::KeyModifiers::NONE
                        } => { 
                            if index > 1 {
                                index -=1 ;
                            }
                        },

                        _ => {
                            //empty, others keys are left useless
                        },
                    }
                    clear_screen_alternate();
                    if event.code == KeyCode::Right || event.code == KeyCode::Left || event.code == KeyCode::Up || event.code == KeyCode::Down{
                        println!("{:?}, index : {} \r", event, index);
                        print_item(index as usize, &mut items);
                    }
                    // println!("{:?}, index : {} \r", event, index);
                };
            } else {
                //lL
                // println!("No input yet\r");}
            } 
            // println!("end");
        
        }
        // terminal::disable_raw_mode()?;
        // print!(">>>");
        // let mut input = String::new();
        // io::stdout().flush().unwrap();
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("unable to read line");
        // println!("input : {}", input);
    }
}

fn clear_screen_alternate() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //for use within alternate screen
}

fn print_item(index: usize, items: &mut Vec<&str>){
    println!();
    // let item = vec!["item_1", "item_2", "item_3", "item_4", "item_5"];
    for(ind, ele) in items.iter().enumerate() {
        // let ind = usize_to_u16(ind);
        if ind+1 == index {
            println!(" {} < \r", ele.red());
        } else {
            println!(" {} \r", ele);
        }
        // print!(" ({}, {}) " , ele, ind)
    }
    println!("\r");
}


#[allow(dead_code)]
fn usize_to_u16(v: usize) -> u32 {
    v as u32
}
