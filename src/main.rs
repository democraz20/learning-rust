// #[macro_use] extern crate prettytable;
use prettytable::{
    // color,
    // Attr,
    Cell,
    Row,
    Table,
};
use serde::{
    Deserialize, 
    Serialize
};

use serde_json::Result;
// use std::env;
use std::fs;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::env;

#[derive(Serialize, Deserialize)]
struct JsonData {
    table: String,
    desc: String,
    config: Config,
    data: Vec<Item>,
}

#[allow(unused_imports)]
use crossterm::{
    execute,
    style::{
        Color, 
        Print, 
        ResetColor, 
        SetBackgroundColor, 
        SetForegroundColor
    },
    ExecutableCommand,
    event,
};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    title_style: String,
    index_style: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    username: String,
    password: String,
    status: bool,
    finished: bool,
}

#[allow(unused_must_use, unused_variables)]
// fn main() -> Result<()> {
fn main() {
    // Create the table
    // let mut num: i32 = 1;
    let filename = String::from("main.json");

    loop {
        let current_dir = env::current_dir().unwrap();
        // print!("{}$", current_dir.display());
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("{}", current_dir.display())),
            ResetColor
        );
        print!("$ ");
        // execute!(
        //     stdout(),
        //     Set
        // );
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        dbg!(&input);
        input.pop();
        println!("[debug] pop occurs");
        dbg!(&input);
        // input = input.to_string().trim();
        let commands = input.split(" ");
        let cmd = commands.collect::<Vec<&str>>();
        dbg!(&cmd);
        let lenght = cmd.len();
        // println!("{:?}", cmd);
        let main_cmd = cmd[0].to_lowercase();
        if main_cmd == "printtb" {
            print_table(&filename);
        }
        
        //defaults
        else if main_cmd == "exit" { break;
        } else if main_cmd == "" {}
        else {println!("Unrecognized command : {}", main_cmd);}; //might revision
    }
        
    // Ok(())
}

// fn print_table
fn print_table(filename: &str) -> Result<()> {
    let mut table = Table::new();
    

    // let contents = fs::read_to_string(filename)
    //     //
    //     .expect("Something went wrong reading th e file");

    // let data: JsonData = serde_json::from_str(&contents).unwrap();
    let data = parse(filename);

    let title_style = data.config.title_style;
    let index_style = data.config.index_style;
    println!("{}, {}", data.table, data.desc);
    // for i in data.data.iter() {
    //     print!("data : {:?}", i);
    //     println!();
    // }
    println!();
    // Add a row per time
    table.add_row(Row::new(vec![
        Cell::new("Index").style_spec(&title_style),
        Cell::new("Username").style_spec(&title_style),
        Cell::new("Password").style_spec(&title_style),
        Cell::new("Status").style_spec(&title_style),
        Cell::new("finished").style_spec(&title_style),
    ]));
    #[allow(unused_variables)]
    for (i, el) in data.data.iter().enumerate() {
        // print!("data : {:?}, index : {}", el, i);
        // println!();
        let mut _status_style;
        let mut _finished_style;
        if data.data[i].status {
            _status_style = "bFg";
        } else {
            _status_style = "bFr";
        }
        if data.data[i].finished {
            _finished_style = "bFg";
        } else {
            _finished_style = "bFr"
        }
        table.add_row(Row::new(vec![
            Cell::new(&(i + 1).to_string()).style_spec(&index_style),
            Cell::new(&data.data[i].username),
            Cell::new(&data.data[i].password),
            Cell::new(&(data.data[i].status).to_string())
                //
                .style_spec(_status_style),
            Cell::new(&(data.data[i].finished).to_string())
                //
                .style_spec(_finished_style),
        ]));
    }
    // Print the table to stdout
    table.printstd();
    Ok(())
}

fn parse(filename: &str) -> JsonData {

    let contents = fs::read_to_string(filename)
        //
        .expect("Something went wrong reading the file");
    let data: JsonData = serde_json::from_str(&contents).expect(&format!(
        "Failed to parse json file, Make sure the json file is correct ({})", filename
    ));
    data
}