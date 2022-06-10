// use std::fs::File;
// use std::io::prelude::*;
// use std::fs::OpenOptions;
// use std::io::Read;
// use std::io::Write;
// use std::fs;
extern crate serde_json;

use serde_json::{Result, Value};

fn main() {
    // let data = r#"
    // {
    //     "id": "001",
    //     "name": "Admin",
    //     "ops": [
    //         {
    //             "ops_id"="001",
    //             "desc"="test description",
    //             "amount"=100
    //         },
    //         {
    //             "ops_id"="002",
    //             "desc"="test description",
    //             "amount"=-50
    //         }
    //     ]
    // }
    //"#;
    let data = r#"
    {
        "name": "Admin",
        "is_admin": true
    }"#;
    let json = serde_json::from_str(data);
    if json.is_ok(){
        println!("json is ok");
        let p: Value = json;
        println!("id : {}, ops : {}", p["name"].as_str().unwrap(), p["is_admin"].as_str().unwrap());
    } else {
        println!("unable to parse json");
    }
}

// fn ex(data: &str)-> Result<Value> {
//     let json: Value = serde_json::from_str(data)?;
//     if json.is_ok(){
//         println!("json is ok");
//         let p = json.as_str().unwrap();
//         println!("id : {}, ops : {}", p["id"], p["ops"]);
//     } else {
//         println!("unable to parse json");
//     }
//     println!("{}", json);
//     println!("id : {}, ops : {}", json["id"], json["ops"]);
//     Ok(())
// }