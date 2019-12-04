#![allow(unused_must_use)]
extern crate json;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
use serde_json::{Value};
use std::env;
use std::collections::HashMap;

////unused... For now
//use std::process::Command;
//use std::io::{stdout, Read, Write, stdin};

/*
* Struct used to store parts of the json request
*/
#[derive(Deserialize, Debug, Serialize)]
pub struct JsonReq {
    jsonrpc: String,
    method: String,
    params: HashMap<String, String>,
    id: i8,
}

/*
* Send a json request to given url 
*/
fn send_request(request_url: &str, req: &JsonReq) -> reqwest::Result<()> {
    let mut response = reqwest::Client::new()
        .post(request_url)
        .json(&req)
        .send()?;

    let out_text = response.text()?;
    let out_slice = out_text.as_str();
    let out_json: Value = serde_json::from_str(&out_slice).unwrap();
    println!("{}", out_json);

    Ok(())
}

/*
* Forge a request with arguments and
* send it at the same time : EPIC
*/
pub fn forge_and_send(request_url: &str, auth_token: &str, method: &str, args: Vec<String>) -> reqwest::Result<()> {
    let mut params_hm: HashMap<String, String> = HashMap::new();
    params_hm.insert("authToken".to_string(), auth_token.to_string());
    for arg in args.iter() {
        let split = arg.split("=");
        let vec: Vec<&str> = split.collect();
            if vec.len() > 1 {
            params_hm.insert(vec[0].to_string(), vec[1].to_string());
        }
    }
    let req = JsonReq{jsonrpc: String::from("2.0"), method: method.to_string(), params: params_hm, id: 1,};
    send_request(&request_url, &req)
}
