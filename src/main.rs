// use std::fs::File;
// use std::io::prelude::*;
// use std::fs::OpenOptions;
// use std::io::Read;
// use std::io::Write;
// use std::fs;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use serde_json::Value as json_value;

#[derive(Serialize, Deserialize)]
struct data {
    name: String,
    id: u32,
    ops: Vec<String>
}

// #[derive(Serialize, Deserialize)]
// struct data_list {
//     data: Vec<>
// }


fn main() {
    let json = r#"
    {
        "name" : "Admin",
        "id" : 1,
        "ops" : ["1", "2", "3"]
    }"#;
    let res = serde_json::from_str(json);

    if res.is_ok(){
        let parsed: data = res.unwrap();
        println!("name : {}", parsed.name);
        // println!("ops  : {:?}", parsed["ops"].as_array().unwrap());
        println!("id   : {}", parsed.id);
        println!("ops  : {:?}", parsed.ops);
    } else {
        println!("error : could not parse json");
    }
}