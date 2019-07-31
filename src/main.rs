use std::{env, thread};
use std::io::{stdout, Write};
use std::time::Duration;

use cairo::debug_reset_static_data;
use log::debug;
use log::info;
use log::warn;
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
    env_logger::init(); // Start logger
    info!("Starting bot");

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), MessageHandler).expect("Failed to create client");
    client.start().expect("Failed to start client");
}

impl EventHandler for MessageHandler {
    fn message(&self, ctx: Context, msg: Message) {
        // Do not allow executing this bot commands by other ones
        if msg.author.bot {
            debug!("Rejected message due to author being bot");
            return;
        }

        let message_content = msg.content.as_str();
        debug!("Message content: {}", message_content);

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
                    warn!("Something wrong happened during sending status response: {}", err);
                    match msg.reply(ctx.http, &format!("Something wrong happened: {}", err.description())) {
                        Err(err) => {
                            warn!("Something wrong happened during sending error message: {}", err);
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
        info!("Bot successfully started");

        thread::sleep(Duration::from_secs(5)); // Wait for cache to load

        thread::spawn(move || {
            info!("Started presence thread");

            loop {
                // Force mutex to unlock before sleep occurs
                {
                    let cache_lock = ctx.cache.read();
                    let guilds_count = cache_lock.guilds.len();
                    info!("Updating guild count presence to {}", guilds_count);
                    ctx.set_presence(Some(Activity::listening(&format!("{} servers", guilds_count))), OnlineStatus::Online);
                }

                thread::sleep(Duration::from_secs(60));
            }
        });
    }
}