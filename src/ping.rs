use serenity::model::channel::Message;

pub fn handler(msg: Message) {
    println!("User \"{}\" pinged us!",msg.author.name);
    let response = msg.channel_id.say("Pong!");
    match response {
        Ok(_) => println!("So we pinged him!"),
        Err(why) => println!("But we are unable to ping him :c {}",why)
    }
}