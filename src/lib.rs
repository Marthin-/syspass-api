#![allow(non_snake_case)]
#![allow(unused_must_use)]
extern crate json;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
use serde_json::{Value};
use std::collections::HashMap;


/*
* Struct used to store parts of the json request
*/
#[derive(Deserialize, Debug, Serialize)]
pub struct JsonReq {
    jsonrpc: String,
    method: String,
    params: HashMap<String, String>,
    id: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct APIResult {
    count: u8,
    itemId: u16,
    result: Vec<Value>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct APIOutput {
    id: u8,
    jsonrpc: String,
    result: APIResult,
    resultcode: u16,
    resultMessage: Option<String>,
}

/*
* Send a json request to given url 
*/
fn send_request(request_url: &str, req: &JsonReq) -> APIOutput {
    let mut response = reqwest::Client::new()
        .post(request_url)
        .json(&req)
        .send().unwrap();

    let out_text = response.text().unwrap();
    let out_slice = out_text.as_str();
//    let out_json: Value = serde_json::from_str(&out_slice).unwrap();
//    out_json
    let api_return: APIOutput = serde_json::from_str(&out_slice).unwrap();
    api_return
}

/*
* Forge a request with arguments and
* send it at the same time : EPIC
*/
pub fn forge_and_send(request_url: &str, auth_token: &str, method: &str, args: Vec<String>) -> APIOutput {
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
    let reply_json: APIOutput = send_request(&request_url, &req);
    reply_json
}
