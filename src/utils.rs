use std::io;
use std::io::stdout;
use std::io::Write;


use crossterm::{
    execute,
    cursor::MoveTo,
};

pub fn is_real_num(num: &str) -> bool {
    match num.trim().parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn alert_screen() {
    clear_screen();
    let vertical = "═";
    let horizontal = "║";
    if let Some((w, h)) = term_size::dimensions(){
        let mut temp_w = w;
        execute!(stdout(), MoveTo(0,0));
        while temp_w > 0 {
            print!("{}", vertical);
            temp_w-=1;
        }
        execute!(stdout(),MoveTo(0,usize_to_u16(h)));
        let mut temp_w = w;
        while temp_w > 0 {
            print!("{}", vertical);
            temp_w-=1;
        }
        let mut input = String::new();
        let mut temp_h = h;
        let mut height = 0;
        execute!(stdout(), MoveTo(0,0));
        while temp_h > 0 {
            execute!(stdout(), MoveTo(0,usize_to_u16(height)));
            print!("{}", horizontal);
            temp_h-=1;
            height+=1;
        }
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    } else {
        println!("failed to get terminal size")
    }
    execute!(stdout(),MoveTo(0,0));
    clear_screen();
}

fn usize_to_u16(v: usize) -> u16 {
    v as u16
}

pub fn clear_screen() {
    if let Some((w, mut h)) = term_size::dimensions() {
        // println!("Dimensions : w{}, h{}", w,h);
        execute!(stdout(),MoveTo(0,0));
        while h > 0 {
            let mut temp = w;
            while temp > 0 {
                temp-=1;
                print!(" ");
            }
            println!("");
            h-=1;
        }
        execute!(stdout(),MoveTo(0,0));
    } else {
        println!("Failed to get terminal size.");
    }
}
