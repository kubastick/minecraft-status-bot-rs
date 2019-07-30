use serenity::model::channel::Message;

pub fn handler(msg: Message) {
    println!("User \"{}\" asked for help using \"{}\" command",msg.author.name,msg.content);
    let help_message = r#":white_check_mark: List of commands:


:bulb: !status <server_address> [--text] - Shows graphical server status [--text - Text instead of graphics]
:bulb: !ping - "Pong"

:hammer: Examples:
!status hypixel.net --text
!ping
!status mistylands.net"#;

    println!("Responding to user \"{}\" with help message",msg.author.name);
    match msg.channel_id.say(help_message) {
        Ok(_) => println!("Responded successfully"),
        Err(why) => println!("Failed to respond: {}",why)
    }
}