use std::fs::File;
// use std::io::prelude::*;
// use std::fs::OpenOptions;
use std::io::Read;
// use std::io::Write;
// use std::fs;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

// use serde_json::Value as json_value;

#[derive(Serialize, Deserialize)]
struct Data {
    name: String,
    id: u32,
    ops: Vec<Ops>
}

#[derive(Serialize, Deserialize)]
struct Ops {
    name: String,
    id: u32
}

fn main() {
    // let json = r#"
    // {
    //     "name" : "Admin",
    //     "id" : 1,
    //     "ops" : [
    //         {
    //             "name" : "Add",
    //             "id" : 1
    //         },
    //         {
    //             "name" : "Delete",
    //             "id" : 2
    //         },
    //         {
    //             "name" : "Update",
    //             "id" : 3
    //         }
    //     ]
    // }"#;
    let mut json = String::new();
    
    let test_append = r#"
    {
        "name" : "test_ops",
        "id" : 4
    }"#;

    let mut file = File::open("main.json").unwrap();
    file.read_to_string(&mut json).expect("unable to read file");

    let res = serde_json::from_str(&json);
    let test_ops = serde_json::from_str(&test_append);
    let parsed_test: Ops = test_ops.unwrap();
    let index = 1;
    if res.is_ok(){
        let mut parsed: Data = res.unwrap();
        println!("name : {}", parsed.name);
        println!("id   : {}", parsed.id);
        println!("ops-name  : {}, ops-id : {}", parsed.ops[index].name, parsed.ops[index].id);
        
        parsed.ops.push(parsed_test);
        for i in &parsed.ops{
            println!("{}, {}", i.name, i.id);
        }
        match std::fs::write(
            "test.json",
            serde_json::to_string_pretty(&parsed).unwrap()
        ) 
        {
            Ok(_) => println!("Success!"),
            Err(e) => println!("Error: {}", e),
        }
        // parsed.ops.pop();
    } else {
        println!("error : could not parse json");
    }
}