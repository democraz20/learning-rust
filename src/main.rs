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
    
    let mut file = File::open("main.json").unwrap();
    file.read_to_string(&mut json).expect("unable to read file");

    let res = serde_json::from_str(&json);
    let index = 1;
    if res.is_ok(){
        let parsed: Data = res.unwrap();
        println!("name : {}", parsed.name);
        // println!("ops  : {:?}", parsed["ops"].as_array().unwrap());
        println!("id   : {}", parsed.id);
        println!("ops-name  : {}, ops-id : {}", parsed.ops[index].name, parsed.ops[index].id);
    } else {
        println!("error : could not parse json");
    }
}