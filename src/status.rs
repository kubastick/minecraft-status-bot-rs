use std::error::Error;
use std::fs::File;
use std::path::Path;

use cairo::{Context, FontFace, FontSlant, FontWeight, ImageSurface};
use serenity::model::channel::Message;

use super::mcsrvstat::ServerStatus;

pub fn handler(msg: Message) -> Result<(), Box<dyn Error>> {
    println!("User \"{}\" asked us for server status using \"{}\" command", msg.author.name, msg.content);
    let server_address = msg
        .content
        .replace("!status", "")
        .replace("--text", "")
        .trim()
        .to_owned();

    // Tell them, that we are starting to work
    msg.channel_id.say("Ok, I'm going to check this minecraft server IP!")?;
    msg.channel_id.broadcast_typing()?;

    // Now query for server status
    match ServerStatus::get_server_status(server_address.as_str()) {
        Ok(status) => {
            match msg.channel_id.say("Here we go!") {
                Ok(_) => println!("We have told them, that we are generating message"),
                Err(why) => println!("We have run into trouble {}", why)
            }
            // Check if we've got --text flag
            if msg.content.contains("--text") {
                println!("We've got --text flag, so we are sending text message");

                let text_status = format!(
                    r"Players online: `{}` \ `{}`
MOTD:           `{}`
Version:        `{}`",
                    status.players_online,
                    status.players_max,
                    status.motd.trim(),
                    status.version.trim()
                );

                msg.channel_id.say(text_status)?;

                Ok(())
            } else {
                // So --text flag is no present
                // Time to generate image!
                let background_image: ImageSurface = load_png_image(Path::new("src/assets/background.png"))?;
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

                msg.channel_id.send_files(vec![(image_buf.as_slice(), "server_status.png")], |f| { f })?;

                Ok(())
            }
        }
        Err(_) => {
            msg.channel_id.say("Sorry, but I can't find minecraft server with these ip :c")?;

            Ok(())
        }
    }
}

fn load_png_image(image_path: &Path) -> Result<ImageSurface, Box<dyn Error>> {
    let mut image_file = File::open(image_path)?;
    let result = ImageSurface::create_from_png(&mut image_file)?;

    Ok(result)
}