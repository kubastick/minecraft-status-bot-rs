use serenity::model::channel::Message;
use super::mcsrvstat;
use super::mcsrvstat::ServerStatus;
use lazy_static::lazy_static;
use jpeg_decoder::{Decoder, PixelFormat};
use cairo::{ImageSurface, Format, Context};
use std::io::{BufReader, Read, ErrorKind};
use std::fs::File;
use std::path::Path;
use std::error::Error;


pub fn handler(msg: Message) {
    println!("User \"{}\" asked us for server status using \"{}\" command", msg.author.name, msg.content);
    let server_address = msg.content.replace("!status", "").replace("--text", "");
    // Tell them, that we are starting to work
    let task_accepted_result = msg.channel_id.say("Ok, I'm going to check this minecraft server IP!");
    match task_accepted_result {
        Ok(_) => println!("We told them, that we are starting to work"),
        Err(why) => println!("We can't tell him that we are starting to work {}", why)
    }
    // Now query for server status
    match mcsrvstat::get_server_status(server_address.as_str()) {
        Ok(status) => {
            match msg.channel_id.say("Here we go!") {
                Ok(_) => println!("We have told them, that we are generating message"),
                Err(why) => println!("We have run into trouble {}", why)
            }
            // Check if we've got --text flag
            if msg.content.contains("--text") {
                println!("We've got --text flag, so we are sending text message");
                match msg.channel_id.say(status.to_string()) {
                    Ok(_) => println!("We have send result, soo goodbye!!"),
                    Err(why) => println!("We can't tell him results :c {}", why)
                }
            } else {
                // So --text flag is no present
                // Time to generate image!
                let background_image = load_jpg_image(Path::new("/assets/background")).expect("failed to load background image");
                let drawing_context = Context::new(&background_image);
                drawing_context.show_text("abc");
                
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

fn load_jpg_image(image_path: &Path) -> Result<ImageSurface, Box<dyn Error>> {
    let img = File::open(image_path)?;
    let mut decoder = Decoder::new(img);
    let decoded_image = decoder.decode()?;
    let image_info = decoder.info().ok_or(std::io::Error::new(ErrorKind::InvalidData,"missing jpg info"))?;

    let image_surface = ImageSurface::create_for_data(
        decoded_image,
        Format::Rgb24,
        image_info.width as i32,
        image_info.height.into(),
        8
    );



    Ok(image_surface.ok().unwrap())
}