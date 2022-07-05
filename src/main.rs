// #[macro_use] extern crate prettytable;
use prettytable::{
    color, 
    Attr, Cell, Row, Table
};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize)]
struct jsondata {
    table: String,
    desc: String,
    data: Vec<item>
}

#[derive(Serialize, Deserialize, Debug)]
struct item {
    username: String,
    password: String
}


fn main() -> Result<()>{
    // Create the table
    let mut num: i32 = 1;
    let mut table = Table::new();
    let title_style = "bFY";
    let index_style = "bFB";
    let filename = "main.json";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let data: jsondata = serde_json::from_str(&contents)?;

    println!("{}, {}", data.table, data.desc);
    for i in data.data.iter() {
        print!("data : {:?}", i);
        println!();
    }
    println!();
    // Add a row per time
    table.add_row(Row::new(vec![
        Cell::new("Index")
            .style_spec(title_style),
        Cell::new("Username")
            .style_spec(title_style),
        Cell::new("Password")
            .style_spec(title_style),
    ]));
    table.add_row(Row::new(vec![
        Cell::new(&num.to_string())
            //
            .style_spec(index_style),
        Cell::new(&data.data[0].username),
        Cell::new(&data.data[0].password),
    ]));
    // A more complicated way to add a row:
    num += 1;
    table.add_row(Row::new(vec![
        Cell::new(&num.to_string())
            //
            .style_spec(index_style),
        Cell::new(&data.data[1].username),
        Cell::new(&data.data[1].password),
    ]));
    
    // Print the table to stdout
    table.printstd();
    Ok(())
}
