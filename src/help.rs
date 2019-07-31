use log::info;
use log::warn;
use serenity::client::Context;
use serenity::model::channel::Message;

pub fn handler(ctx: Context, msg: Message) {
    info!("User \"{}\" asked for help using \"{}\" command", msg.author.name, msg.content);
    let help_message = r#":white_check_mark: List of commands:


:bulb: !status <server_address> [--text] - Shows graphical server status [--text - Text instead of graphics]
:bulb: !ping - "Pong"

:hammer: Examples:
!status hypixel.net --text
!ping
!status mistylands.net"#;

    match msg.channel_id.say(ctx.http, help_message) {
        Ok(_) => {}
        Err(why) => warn!("Failed to send message: {}", why)
    }
}