use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::RwLock;

use cairo::{Context, FontFace, FontSlant, FontWeight, ImageSurface};
use log::debug;
use log::info;
use serenity::model::channel::Message;

use lazy_static::lazy_static;

use super::mcsrvstat::ServerStatus;
use super::SETTINGS;

lazy_static! {
    static ref BACKGROUND_DATA: RwLock<Vec<u8>> = {
        let image_path = SETTINGS.read().unwrap().get_str("BACKGROUND_PNG_LOCATION").unwrap();
        let mut image_file = File::open(image_path).unwrap();
        let mut buf = Vec::new();
        image_file.read_to_end(&mut buf).unwrap();

        RwLock::new(buf)
    };
}

pub fn handler(ctx: serenity::client::Context, msg: Message) -> Result<(), Box<dyn Error>> {
    info!("User \"{}\" executed fetching status using \"{}\" command", msg.author.name, msg.content);

    let server_address = msg
        .content
        .replace("!status", "")
        .replace("--text", "")
        .trim()
        .to_owned();

    // Tell them, that we are starting to work
    msg.channel_id.say(&ctx.http, "Ok, I'm going to check this minecraft server IP!")?;
    msg.channel_id.broadcast_typing(&ctx.http)?;

    // Now query for server status
    match ServerStatus::get_server_status(server_address.as_str()) {
        Ok(status) => {
            msg.channel_id.say(&ctx.http, "Here we go!")?;
            // Check if we've got --text flag
            if msg.content.contains("--text") {
                debug!("Sending text variant of the status");

                let text_status = format!(
                    r"Players online: `{}` \ `{}`
MOTD:           `{}`
Version:        `{}`",
                    status.players_online,
                    status.players_max,
                    status.motd.trim(),
                    status.version.trim()
                );

                msg.channel_id.say(ctx.http, text_status)?;

                Ok(())
            } else {
                // So --text flag is no present
                // Time to generate image!
                
                let background_image: ImageSurface = create_background_surface()?;
                let drawing_context = Context::new(&background_image);

                // Draw MOTD
                drawing_context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                drawing_context.set_font_size(50.0);
                drawing_context.move_to(50.0, 75.0);
                drawing_context.set_font_face(&FontFace::toy_create("Minecraftia", FontSlant::Normal, FontWeight::Normal));
                drawing_context.show_text(&status.motd);

                // Draw IP
                drawing_context.set_font_size(20.0);
                drawing_context.move_to(50.0, 140.0);
                drawing_context.show_text(&server_address);

                // Draw player count
                drawing_context.set_font_size(35.0);
                drawing_context.move_to(50.0, 265.0);
                drawing_context.set_source_rgb(0.0, 1.0, 0.0);
                let players_text = format!("{} players online of {} max", &status.players_online, &status.players_max);
                drawing_context.show_text(&players_text);

                // Draw version
                drawing_context.move_to(50.0, 335.0);
                drawing_context.set_source_rgb(1.0, 1.0, 0.0);
                drawing_context.show_text(&format!("Version: {}", &status.version));

                // Draw about
                drawing_context.set_source_rgb(1.0, 1.0, 1.0);
                drawing_context.set_font_size(10.0);
                drawing_context.move_to(1120.0 - 370.0, 700.0 - 50.0);
                drawing_context.show_text("Generated using Minecraft Server Status Bot [Discord]");
                drawing_context.move_to(1120.0 - 370.0, 700.0 - 30.0);
                drawing_context.show_text("Generated by Rust programming language using Cairo");

                // Draw player list
                match status.player_list {
                    Some(player_list) => {
                        if let 1...10 = player_list.len() {
                            debug!("Drawing {} player list", player_list.len());

                            drawing_context.set_font_size(15.0);
                            drawing_context.move_to(50.0, 400.0);
                            drawing_context.show_text("Player list:");

                            let mut i = 0;
                            for player in player_list {
                                drawing_context.move_to(50.0, 425.0 + i as f64 * 25.0);
                                drawing_context.show_text(&player);
                                i += 1;
                            }
                        }
                    }

                    _ => {}
                }


                let mut image_buf: Vec<u8> = vec![];

                background_image.write_to_png(&mut image_buf)?;

                msg.channel_id.send_files(&ctx.http, vec![(image_buf.as_slice(), "server_status.png")], |f| { f })?;

                Ok(())
            }
        }
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Sorry, but I can't find minecraft server with these ip :c")?;

            Ok(())
        }
    }
}

fn create_background_surface() -> Result<ImageSurface, Box<dyn Error>> {
    let background_lock = BACKGROUND_DATA.read()?;
    let mut data_ref = background_lock.as_slice();
    let result = ImageSurface::create_from_png(&mut data_ref)?;

    Ok(result)
}