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

pub fn alert_screen() {
    clear_screen();
    if let Some((mut w, mut h)) = term_size::dimensions() {
        execute!(stdout(),MoveTo(usize_to_u16(w/4),usize_to_u16(h/4)));
        let mut temp = w/2;
        while temp > 0 {
            temp-=1;
            print!("_");
        }
        print!("{}", w/2);
        execute!(stdout(),MoveTo(usize_to_u16(w/4),usize_to_u16(h-(h/4))));
        let mut temp = w/2;
        while temp > 0 {
            temp-=1;
            print!("-");
        }
        print!("{}", w/2);
    } else {
        println!("failed to get terminal size")
    }
    execute!(stdout(),MoveTo(0,0));

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
