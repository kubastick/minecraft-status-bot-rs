use std::error::Error;
use std::io;
use std::prelude::v1::Result;

use reqwest;
use serde_json::{Value};
use serde_json;

#[derive(Debug)]
pub struct ServerStatus {
    motd: String,
    version: String,
    players_online: i64,
    players_max: i64,
}


pub fn get_server_status(server_status: &str) -> Result<ServerStatus, Box<Error>> {
    let mut request = reqwest::get((String::from("https://api.mcsrvstat.us/1/") + server_status).as_str())?;

    if request.status() != 200 {
        return Result::Err(Box::new(io::Error::new(io::ErrorKind::InvalidData,"Server returned non-200 response code")));
    }

    let json: Value = serde_json::from_str(request.text()?.as_str())?;
    let status = ServerStatus {
        motd: String::from(json["motd"]["clean"][0].clone().as_str().unwrap_or("").trim()),
        version: String::from(json["version"].clone().as_str().unwrap_or("")),
        players_online: json["players"]["online"].as_i64().unwrap_or(0),
        players_max: json["players"]["max"].clone().as_i64().unwrap_or(0),
    };
    debug_assert!(status.players_max>0);
    Result::Ok(status)
}

