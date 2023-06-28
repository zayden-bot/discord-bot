use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use crate::models::{GoodMorningImage, GoodNightImage};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// pub fn create_post(conn: &mut PgConnection, url: &str) -> GoodMorningImage {
//     use crate::schema::good_morning_images;
//
//     let new_image = NewImage {
//         image_url: url,
//     };
//
//     diesel::insert_into(good_morning_images::table)
//         .values(&new_image)
//         .get_result(conn)
//         .expect("Error saving new post")
// }

pub fn get_good_morning_images() -> Vec<String> {
    use crate::schema::good_morning_images::dsl::*;

    let conn = &mut establish_connection();

    let results = good_morning_images
        .select(GoodMorningImage::as_select())
        .load::<GoodMorningImage>(conn)
        .expect("Error loading posts");

    results.iter().map(|s| s.image_url.clone()).collect()
}

pub fn get_good_night_images() -> Vec<String> {
    use crate::schema::good_night_images::dsl::*;

    let conn = &mut establish_connection();

    let results = good_night_images
        .select(GoodNightImage::as_select())
        .load::<GoodNightImage>(conn)
        .expect("Error loading posts");

    results.iter().map(|s| s.image_url.clone()).collect()
}