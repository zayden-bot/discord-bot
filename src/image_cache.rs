use serenity::prelude::TypeMapKey;
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

#[derive(Debug)]
pub struct ImageCache {
    pub good_morning_images: Vec<PathBuf>,
    pub good_night_images: Vec<PathBuf>,
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
        }
    }
}

impl TypeMapKey for ImageCache {
    type Value = ImageCache;
}
