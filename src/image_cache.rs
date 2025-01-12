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

fn create_character_map(images: Vec<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
    let mut character_map = HashMap::new();

    for image in images {
        let character = image
            .iter()
            .nth(2)
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let entry = character_map.entry(character).or_insert_with(Vec::new);
        entry.push(image);
    }

    character_map
}

#[derive(Debug)]
pub struct ImageCache {
    pub good_morning_images: Vec<PathBuf>,
    pub good_night_images: Vec<PathBuf>,
    pub character_map: HashMap<String, Vec<PathBuf>>,
}

impl ImageCache {
    pub fn new() -> Self {
        let images = get_images();

        Self {
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
