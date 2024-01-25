use chrono::{Local, NaiveDateTime};
use serenity::prelude::TypeMapKey;
use std::fs;
use std::path::PathBuf;

const GOOD_MORNING_DIR: &str = r"C:\Users\viridian\Documents\zayden\good_morning";
const GOOD_NIGHT_DIR: &str = r"C:\Users\viridian\Documents\zayden\good_night";

fn get_images(dir: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect()
}

pub struct ImageCache {
    pub last_update: NaiveDateTime,
    pub good_morning_images: Vec<PathBuf>,
    pub good_night_images: Vec<PathBuf>,
}
impl ImageCache {
    pub fn new() -> Self {
        Self {
            last_update: Local::now().naive_utc(),
            good_morning_images: get_images(GOOD_MORNING_DIR),
            good_night_images: get_images(GOOD_NIGHT_DIR),
        }
    }

    pub async fn update(&mut self) {
        self.last_update = Local::now().naive_utc();
        self.good_morning_images = get_images(GOOD_MORNING_DIR);
        self.good_night_images = get_images(GOOD_NIGHT_DIR);
    }
}

impl TypeMapKey for ImageCache {
    type Value = ImageCache;
}
