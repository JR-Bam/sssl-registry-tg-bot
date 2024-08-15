use std::path::PathBuf;

use teloxide::{prelude::*, types::InputFile};


pub enum Animal {
    Samoyed,
    Sable,
    SnowLeopard
}

pub async fn get_animal_image(bot: &Bot, msg: &Message, animal: Animal) -> ResponseResult<()> {
    let mut image_path = PathBuf::from("images");
    match animal {
        Animal::Samoyed => {
            image_path.push("sables");
        },
        Animal::Sable => {
            image_path.push("sables");
        },
        Animal::SnowLeopard => {
            image_path.push("sables");
        },
    }

    image_path.push("photo_21_2024-04-10_21-21-20.jpg");
    let photo = InputFile::file(image_path);
    bot.send_photo(msg.chat.id, photo).await?;

    Ok(())
}
