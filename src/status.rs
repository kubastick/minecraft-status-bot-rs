use serenity::model::channel::Message;
use super::mcsrvstat;
use super::mcsrvstat::ServerStatus;
pub fn handler(msg: Message) {
    println!("User \"{}\" asked us for server status using \"{}\" command", msg.author.name, msg.content);
    let server_address = msg.content.replace("!status", "").replace("--text","");
    // Tell them, that we are starting to work
    let task_accepted_result = msg.channel_id.say("Ok, I'm going to check this minecraft server IP!");
    match task_accepted_result {
        Ok(_) => println!("We telled them, that we are starting to work"),
        Err(why) => println!("We can't tell him that we are starting to work {}", why)
    }
    // Now query for server status
    match mcsrvstat::get_server_status(server_address.as_str()) {
        Ok(status) => {
            // TODO: Do something with status
            // Check if we'have passed
        }
        Err(why) => {
            println!("We have an small failure there: {}", why);
            match msg.channel_id.say("Sorry, but I can't find minecraft server with these ip :c") {
                Ok(_) => println!("Sended message about unability to find server"),
                Err(why) => println!("We are unable to even send failure message (sadly) {}",why)
            }
        }
    }
}

fn generate_text_repr(status: ServerStatus) {
    let format
}