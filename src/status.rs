use serenity::model::channel::Message;
use super::mcsrvstat;
use super::mcsrvstat::ServerStatus;

pub fn handler(msg: Message) {
    println!("User \"{}\" asked us for server status using \"{}\" command", msg.author.name, msg.content);
    let server_address = msg.content.replace("!status", "").replace("--text", "");
    // Tell them, that we are starting to work
    let task_accepted_result = msg.channel_id.say("Ok, I'm going to check this minecraft server IP!");
    match task_accepted_result {
        Ok(_) => println!("We telled them, that we are starting to work"),
        Err(why) => println!("We can't tell him that we are starting to work {}", why)
    }
    // Now query for server status
    match mcsrvstat::get_server_status(server_address.as_str()) {
        Ok(status) => {
            match msg.channel_id.say("Here we go!") {
                Ok(_) => println!("We have telled them, that we are generating message"),
                Err(why) => println!("We have run into trouble {}",why)
            }
            // TODO: Do something with status
            // Check if we've got --text flag
            if msg.content.contains("--text") {
                println!("We've got --text flag, so we are sending text message");
                match msg.channel_id.say(status.to_string()) {
                    Ok(_) => println!("We have send result, soo goodbye!!"),
                    Err(why) => println!("We can't tell him results {}", why)
                }
            }
        }
        Err(why) => {
            println!("We have an small failure there: {}", why);
            match msg.channel_id.say("Sorry, but I can't find minecraft server with these ip :c") {
                Ok(_) => println!("Sended message about unability to find server"),
                Err(why) => println!("We are unable to even send failure message (sadly) {}", why)
            }
        }
    }
}