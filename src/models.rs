use sqlx::FromRow;

#[derive(FromRow)]
pub struct Image {
    pub id: i32,
    pub image_url: String,
}