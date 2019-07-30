use std::env;

use serenity::Client;
use serenity::prelude::Context;
use serenity::prelude::EventHandler;
use serenity::model::channel::Message;
use serenity::model::prelude::gateway::Ready;

struct MessageHandler;

mod help;
mod ping;
mod status;
pub mod mcsrvstat;

fn main() {
    print!("Starting bot...");
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), MessageHandler).expect("Failed to create client");
    client.start().expect("Failed to start client");
}

impl EventHandler for MessageHandler {
    fn message(&self, _: Context, msg: Message) {
        let message_content = msg.content.as_str();
        if message_content.starts_with("!help") {
            help::handler(msg);
            return
        }
        if message_content.starts_with("!ping") {
            ping::handler(msg);
            return
        }
        if message_content.starts_with("!status") {
            status::handler(msg);
            return;
        }
    }

    fn ready(&self, _: Context, _: Ready) {
        println!("OK!");
    }
}