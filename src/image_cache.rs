use chrono::{Local, NaiveDateTime};
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;

fn get_images() -> Vec<PathBuf> {
    WalkDir::new("images")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn create_character_map(images: Vec<PathBuf>) -> HashMap<Box<str>, Vec<PathBuf>> {
    let mut character_map = HashMap::new();

    for image in images {
        let character = image
            .iter()
            .nth(2)
            .and_then(|s| s.to_str())
            .map(Box::from)
            .unwrap_or(Box::from("unknown"));

        let entry = character_map.entry(character).or_insert_with(Vec::new);
        entry.push(image);
    }

    character_map
}

#[derive(Debug, Clone)]
pub struct ImageCache {
    pub last_update: NaiveDateTime,
    pub good_morning_images: Vec<PathBuf>,
    pub good_night_images: Vec<PathBuf>,
    pub character_map: HashMap<Box<str>, Vec<PathBuf>>,
}

impl ImageCache {
    pub fn new() -> Self {
        let images = get_images();

        Self {
            last_update: Local::now().naive_utc(),
            good_morning_images: images
                .iter()
                .filter(|p| p.iter().nth(1).and_then(|s| s.to_str()) == Some("good_morning"))
                .cloned()
                .collect(),
            good_night_images: images
                .iter()
                .filter(|p| p.iter().nth(1).and_then(|s| s.to_str()) == Some("good_night"))
                .cloned()
                .collect(),
            character_map: create_character_map(images),
        }
    }
}

impl TypeMapKey for ImageCache {
    type Value = ImageCache;
}
