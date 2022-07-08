use std::{thread, time};

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use chrono::prelude::*;

#[allow(unused_imports)]
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

#[allow(dead_code)]
const DURATION: std::time::Duration = time::Duration::from_secs(1200);
const FILENAME: &str = "logs.txt";

fn main() -> std::io::Result<()> {
    let mut sys = System::new_all();
    loop {
        sys.refresh_all();
        println!("=> system:");
        // RAM and swap information:
        let total_mem: f32 = sys.total_memory() as f32 / 1048576.0;
        let used_mem: f32 = sys.used_memory() as f32 / 1048576.0;
        let total_swap: f32 = sys.total_swap() as f32 / 1048576.0;
        let used_swap: f32 = sys.used_swap() as f32 / 1048576.0;

        print!("total memory: {} GB", total_mem);
        print!(", used memory: {} GB", used_mem);
        print!(", total swap: {} GB", total_swap);
        println!(", used swap: {} GB", used_swap);

        let local = Local::now();
        let local_time = local.to_string();
        let local_time = local_time.split(" ");
        let mut local_time = local_time.collect::<Vec<&str>>();
        local_time.pop();
        println!("Current time : {:?}", local_time);

        let final_write = format!(
            "{}, {}, {}, {} \n  Logged At : {:?}",
            total_mem, used_mem, total_swap, used_swap, local_time
        )
        .to_string();

        if Path::new(FILENAME).exists() {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(FILENAME)
                .unwrap();
            if let Err(e) = writeln!(file, "{}", &final_write) {
                eprintln!("Couldn't write to file: {}", e);
            }
        } else {
            File::create(FILENAME)?;
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(FILENAME)
                .unwrap();
            if let Err(e) = writeln!(
                file,
                "{}",
                format!("==( logs )== \nstarted at : {:?} \n==( logs )==", local_time)
            ) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        thread::sleep(DURATION);
    }
}
