use std::{env, thread};
use std::io::{stdout, Write};
use std::time::Duration;

use serenity::Client;
use serenity::model::channel::Message;
use serenity::model::gateway::Activity;
use serenity::model::prelude::gateway::Ready;
use serenity::model::user::OnlineStatus;
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
    fn message(&self, ctx: Context, msg: Message) {
        // Do not allow executing this bot commands by other ones
        if msg.author.bot {
            return;
        }

        let message_content = msg.content.as_str();
        if message_content.starts_with("!help") {
            help::handler(ctx, msg);
            return;
        }
        if message_content.starts_with("!ping") {
            ping::handler(ctx, msg);
            return;
        }
        if message_content.starts_with("!status") {
            match status::handler(ctx.clone(), msg.clone()) {
                Err(err) => {
                    match msg.reply(ctx.http, &format!("Something wrong happened: {}", err.description())) {
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

    fn ready(&self, ctx: Context, _: Ready) {
        println!("OK!");

        thread::spawn(move || {
            loop {

                // Force mutex to unlock before sleep occurs
                {
                    let cache_lock = ctx.cache.read();
                    let guilds_count = cache_lock.guilds.len();
                    ctx.set_presence(Some(Activity::listening(&format!("{} servers", guilds_count))), OnlineStatus::Online);
                }

                thread::sleep(Duration::from_secs(60));
            }
        });
    }
}