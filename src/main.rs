use std::{thread, time};

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::fs;
use std::io;

use chrono::prelude::*;

#[allow(unused_imports)]
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use serde::{
    Deserialize, 
    Serialize
};
/*
#[allow(dead_code)]
const DURATION: std::time::Duration = time::Duration::from_secs(1200);
const FILENAME: &str = "logs.txt";
*/
#[derive(Serialize, Deserialize)]
struct Config {
    duration: u64,
    filename: String
}

// type Error = Box<dyn std::error::Error>;
 
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

        let config = parse_config("config.json")?;

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

        if Path::new(&config.filename).exists() {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(config.filename)
                .unwrap();
            if let Err(e) = writeln!(file, "{}", &final_write) {
                eprintln!("Couldn't write to file: {}", e);
            }
        } else {
            File::create(&config.filename)?;
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(config.filename)
                .unwrap();
            if let Err(e) = writeln!(
                file,
                "{}",
                format!("==( logs )== \nstarted at : {:?} \n==( logs )==", local_time)
            ) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        thread::sleep(time::Duration::from_secs(config.duration));
    }
}

fn parse_config(filename: &str) -> Result<Config, std::io::Error> {

    let contents = fs::read_to_string(filename)?;
    let parse: Config = serde_json::from_str(&contents)?;
    Ok(parse)
    //     //
    //     .expect("Something went wrong reading the file");
    // let data: Config = serde_json::from_str(&contents).expect(&format!(
    //     "Failed to parse json file, Make sure the json file is correct ({})", filename
    // ));
    // data
}