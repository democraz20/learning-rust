use std::io::stdout;

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
