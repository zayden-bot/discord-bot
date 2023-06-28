use crate::schema::good_morning_images;
use crate::schema::good_night_images;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = good_morning_images)]
pub struct GoodMorningImage {
    pub id: i32,
    pub image_url: String,
}

#[derive(Insertable)]
#[diesel(table_name = good_morning_images)]
pub struct NewGoodMorningImage<'a> {
    pub image_url: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = good_night_images)]
pub struct GoodNightImage {
    pub id: i32,
    pub image_url: String,
}

#[derive(Insertable)]
#[diesel(table_name = good_night_images)]
pub struct NewGoodNightImage<'a> {
    pub image_url: &'a str,
}