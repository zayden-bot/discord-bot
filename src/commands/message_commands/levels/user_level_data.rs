use chrono::NaiveDateTime;

pub struct UserLevelData {
    pub id: i64,
    pub total_xp: i32,
    pub last_xp: NaiveDateTime,
}
