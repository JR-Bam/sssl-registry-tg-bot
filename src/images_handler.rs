use std::{fs, path::PathBuf};
use rand::{seq::SliceRandom, thread_rng};
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
            image_path.push("samoyeds");
        },
        Animal::Sable => {
            image_path.push("sables");
        },
        Animal::SnowLeopard => {
            image_path.push("snowleopards");
        },
    }

    let photos = fs::read_dir(&image_path)?
        .filter_map(|entry|{
            let entry = entry.ok()?;
            let path = entry.path();
            let extension = path.extension()?;

            if extension == "png" || extension == "jpg" || extension == "jpeg" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>();

    let selected_photo = photos.choose(&mut thread_rng()).unwrap();
    image_path.push(selected_photo.file_name().unwrap());

    if !image_path.exists() {
        let _ = bot.send_message(msg.chat.id, format!("Path doesn't exist: {:?}", image_path)).await?;
    } else {
        let photo = InputFile::file(image_path);
        bot.send_photo(msg.chat.id, photo).await?;
    }

    Ok(())
}
