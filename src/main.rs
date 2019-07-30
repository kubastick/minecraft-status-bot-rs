use std::env;
use std::io::{stdout, Write};

use serenity::Client;
use serenity::model::channel::Message;
use serenity::model::prelude::gateway::Ready;
use serenity::prelude::Context;
use serenity::prelude::EventHandler;

struct MessageHandler;

mod help;
mod ping;
mod status;
pub mod mcsrvstat;

fn main() {
    print!("Starting bot...");
    stdout().flush().unwrap(); // Force pushing above println to the terminal

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), MessageHandler).expect("Failed to create client");
    client.start().expect("Failed to start client");
}

impl EventHandler for MessageHandler {
    fn message(&self, _: Context, msg: Message) {
        // Do not allow executing this bot commands by other ones
        if msg.author.bot {
            return
        }
        
        let message_content = msg.content.as_str();
        if message_content.starts_with("!help") {
            help::handler(msg);
            return;
        }
        if message_content.starts_with("!ping") {
            ping::handler(msg);
            return;
        }
        if message_content.starts_with("!status") {
            match status::handler(msg.clone()) {
                Err(err) => {
                    match msg.reply(&format!("Something wrong happened: {}", err.description())) {
                        Err(err) => {
                            println!("{}", err);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            return;
        }
    }

    fn ready(&self, _: Context, _: Ready) {
        println!("OK!");
    }
}