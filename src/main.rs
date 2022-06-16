use std::io;
use std::io::Write;
// use core::ptr::null;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::io::stdout;

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), MoveTo(0, 0),);
    loop {
        let mut input = String::new();
        print!(">>>");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_lowercase();
        if input == "exit" {
            execute!(stdout(), LeaveAlternateScreen)?;
            break;
        } else if input == "hello" {
            println!("world!");
        } else if input == "" {
            continue;
        } else {
            println!("unrecognized input : {}", input);
        }
    }
    Ok(())
}
