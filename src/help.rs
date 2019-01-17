use serenity::model::channel::Message;

pub fn handler(msg: Message) {
    println!("User \"{}\" asked for help using \"{}\" command",msg.author.name,msg.content);
    let help_message = include_str!("./assets/help_message.txt");
    println!("Responding to user \"{}\" with help message",msg.author.name);
    match msg.channel_id.say(help_message) {
        Ok(_) => println!("Responded successfully"),
        Err(why) => println!("Failed to respond: {}",why)
    }
}