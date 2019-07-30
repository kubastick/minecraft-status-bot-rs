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
    pub player_list: Option<Vec<String>>,
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
    pub fn get_server_status(server_status: &str) -> Result<Self, Box<dyn Error>> {
        let mut request = reqwest::get((String::from("https://api.mcsrvstat.us/1/") + server_status).as_str())?;

        if request.status() != 200 {
            return Result::Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Server returned non-200 response code")));
        }

        let json: Value = serde_json::from_str(request.text()?.as_str())?;



        let mut status = Self {
            motd: String::from(json["motd"]["clean"][0].clone().as_str().unwrap_or("").trim()),
            version: String::from(json["version"].clone().as_str().unwrap_or("").trim()),
            players_online: json["players"]["online"].as_i64().unwrap_or(0),
            players_max: json["players"]["max"].clone().as_i64().unwrap_or(0),
            player_list: None,
        };

        let player_list=  json["players"]["list"].as_array();
        match player_list {
            Some(player_list)=>{
                let player_list = player_list
                    .clone()
                    .iter()
                    .map(|v|{v.as_str().unwrap_or("")})
                    .map(String::from)
                    .collect();
                status.player_list = Some(player_list);
            }
            _ => {}
        }

        let status = status;

        debug_assert!(status.players_max > 0);
        debug_assert!(status.players_online <= status.players_max);

        Result::Ok(status)
    }
}

