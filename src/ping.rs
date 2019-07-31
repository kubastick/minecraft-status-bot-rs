use log::info;
use log::warn;
use serenity::client::Context;
use serenity::model::channel::Message;

pub fn handler(ctx: Context, msg: Message) {
    let response = msg.reply(ctx.http, "Pong!");
    match response {
        Ok(_) => {},
        Err(why) => warn!("Error why sending ping response: {}", why)
    }
}