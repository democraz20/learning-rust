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


#[allow(unused_must_use)]
pub fn alert_screen(message: String) -> String{
    let mut input = String::new();
    clear_screen();
    let vertical = "║";
    let horizontal = "═";
    if let Some((w, h)) = term_size::dimensions(){
        let mut temp_w = w;
        //top and bottom
        execute!(stdout(), MoveTo(0,0));
        while temp_w > 0 {
            print!("{}", horizontal);
            temp_w-=1;
        }
        execute!(stdout(),MoveTo(0,usize_to_u16(h)));
        let mut temp_w = w;
        while temp_w > 0 {
            print!("{}", horizontal);
            temp_w-=1;
        }
        let mut temp_h = h;
        let mut height = 0;
        //sides 
        execute!(stdout(), MoveTo(0,0));
        while temp_h > 0 {
            execute!(stdout(), MoveTo(0,usize_to_u16(height)));
            print!("{}", vertical);
            temp_h-=1;
            height+=1;
        }
        let mut temp_h = h;
        let mut height = 0;
        while temp_h > 0 {
            execute!(stdout(), MoveTo(usize_to_u16(w),usize_to_u16(height)));
            print!("{}", vertical);
            temp_h-=1;
            height+=1;
        }
        //corners
        execute!(stdout(), MoveTo(0,0));
        print!("╔");
        execute!(stdout(), MoveTo(usize_to_u16(w), 0));
        print!("╗");
        execute!(stdout(), MoveTo(0, usize_to_u16(h)));
        print!("╚");
        execute!(stdout(), MoveTo(usize_to_u16(w), usize_to_u16(h)));
        print!("╝");

        //title
        execute!(stdout(), MoveTo(1,0));
        print!("[alert_screen]");
        //end 
        execute!(stdout(), MoveTo(usize_to_u16(w/4),usize_to_u16(h-(h/4))));
        print!(">");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    } else {
        println!("failed to get terminal size");
    }
    execute!(stdout(),MoveTo(0,0));
    clear_screen();
    return input;
}

fn usize_to_u16(v: usize) -> u16 {
    v as u16
}

#[allow(unused_must_use)]
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
