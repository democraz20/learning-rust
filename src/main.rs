use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};

use std::io;
use std::process;
use std::io::Write;
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

fn main() -> crossterm::Result<()> {
    let mut index: i32 = 0;
    let index_limit = 5;
    let _clean_up = CleanUp;
    println!("Recording Key Started"); 
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
                            println!("Exiting Program \r");
                            terminal::disable_raw_mode()?;
                            process::exit(1);
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
                            if index > 0 {
                                index -=1 ;
                            }
                        },
                        _ => {
                            //todo
                        },
                    }
                    println!("{:?}, {} \r", event, index);
                };
            } else {
                //lL
                // println!("No input yet\r");}
            } 
            // println!("end");
        
        }
        terminal::disable_raw_mode()?;
        print!(">>>");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read line");
        println!("input : {}", input);
    }
    Ok(())
}


// fn main() -> Result<(), Box<dyn std::error::Error>> {

// enable_raw_mode()?;
// let (tx, rx) = mpsc::channel();
// let tick_rate = Duration::from_millis(200);
// thread::spawn(move || {
//     let mut last_tick = Instant::now();
//     loop {
//         let timeout = tick_rate
//             .checked_sub(last_tick.elapsed())
//             .unwrap_or_else(|| Duration::from_secs(0));

//         if event::poll(timeout).expect("poll works") {
//             if let CEvent::Key(key) = event::read().expect("can read events") { tx.send(Event::Input(key)).expect("can send events");
//             }
//         }

//         if last_tick.elapsed() >= tick_rate {
//             if let Ok(_) = tx.send(Event::Tick) {
//                 last_tick = Instant::now();
//             }
//         }
//     }
// });
// let stdout = io::stdout();
// let backend = CrosstermBackend::new(stdout);
// let mut terminal = Terminal::new(backend)?;
// loop{
// match rx.recv()? {
//     Event::Input(event) => match event {
//         // KeyCode::Char('q') => {
//         //     disable_raw_mode()?;
//         //     terminal.show_cursor()?;
//         //     break;
//         // }
//         // _ => {}
//         KeyEvent {
//             code: KeyCode::Char('q'),
//             modifiers: event::KeyModifiers::CONTROL
//         } => {
//             disable_raw_mode()?;
//             terminal.show_cursor()?;
//             break;
//         },
//         _ => {}
//     }
//     // Event::Tick => {}
// }
// print!("{:?}", event);
// Ok(())
// }
