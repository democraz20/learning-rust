// #[macro_use] extern crate prettytable;
use prettytable::{
    color, 
    Attr, Cell, Row, Table
};
use colored::*;


fn main() {
    // Create the table
    let mut num: i32 = 1;
    let mut table = Table::new();
    let title_style = "bFY";
    let index_style = "bFB";

    println!("{}","=ACCOUNTS=".cyan());
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
        Cell::new("hello"),
        Cell::new("world"),
    ]));
    // A more complicated way to add a row:
    num += 1;
    table.add_row(Row::new(vec![
        Cell::new(&num.to_string())
            //
            .style_spec(index_style),
        Cell::new("foo"),
        Cell::new("bar"),
    ]));

    // Print the table to stdout
    table.printstd();
}
