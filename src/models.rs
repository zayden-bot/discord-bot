use sqlx::FromRow;

#[derive(FromRow)]
pub struct Image {
    pub id: i32,
    pub image_url: String,
}

#[derive(FromRow)]
pub struct SupportFAQ {
    pub id: String,
    pub answer: String,
    pub guild_id: i64,
}

#[derive(FromRow)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub answer: Option<String>,
    pub user_id: i64,
    pub message_id: Option<i64>,
}
