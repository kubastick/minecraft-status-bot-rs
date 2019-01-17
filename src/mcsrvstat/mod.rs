use std::error::Error;
use std::io;
use std::prelude::v1::Result;

use reqwest;
use serde_json::Value;
use serde_json;
use std::fmt;

#[derive(Debug)]
pub struct ServerStatus {
    pub motd: String,
    pub version: String,
    pub players_online: i64,
    pub players_max: i64,
}

impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, include_str!("./assets/status_response_template.txt"), self.players_online,
               self.players_max,
               self.motd,
               self.version)
    }
}

impl ServerStatus {
    pub fn generate_image(&self) {
       // let mut image_surface = ImageSurface::create(Format::Rgb24,)
    }
}


pub fn get_server_status(server_status: &str) -> Result<ServerStatus, Box<Error>> {
    let mut request = reqwest::get((String::from("https://api.mcsrvstat.us/1/") + server_status).as_str())?;

    if request.status() != 200 {
        return Result::Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Server returned non-200 response code")));
    }

    let json: Value = serde_json::from_str(request.text()?.as_str())?;
    let status = ServerStatus {
        motd: String::from(json["motd"]["clean"][0].clone().as_str().unwrap_or("").trim()),
        version: String::from(json["version"].clone().as_str().unwrap_or("").trim()),
        players_online: json["players"]["online"].as_i64().unwrap_or(0),
        players_max: json["players"]["max"].clone().as_i64().unwrap_or(0),
    };
    debug_assert!(status.players_max > 0);
    Result::Ok(status)
}

