use std::env;

extern crate serenity;

use serenity::Client;
use serenity::prelude::Context;
use serenity::prelude::EventHandler;
use serenity::model::channel::Message;

struct MessageHandler;

mod help;

fn main() {
    println!("Starting bot...");
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), MessageHandler).expect("Failed to create file");
    client.start().expect("Failed to start client");
}

impl EventHandler for MessageHandler {
    fn message(&self, _: Context, msg: Message) {
        if msg.content == "!help" {
            help::handler(msg);
        }
    }
}