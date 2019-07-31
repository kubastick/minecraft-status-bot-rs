use serenity::client::Context;
use serenity::model::channel::Message;

pub fn handler(ctx: Context, msg: Message) {
    println!("User \"{}\" pinged us!", msg.author.name);
    let response = msg.channel_id.say(ctx.http, "Pong!");
    match response {
        Ok(_) => println!("So we pinged him!"),
        Err(why) => println!("But we are unable to ping him :c {}", why)
    }
}