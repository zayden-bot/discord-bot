use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::ShopItem;

#[derive(Debug, Default, Deserialize, Serialize, FromRow)]
pub struct GamblingItem {
    pub quantity: i64,
    pub item_id: String,
}

impl From<&ShopItem<'_>> for GamblingItem {
    fn from(value: &ShopItem<'_>) -> Self {
        Self {
            quantity: 0,
            item_id: value.id.to_string(),
        }
    }
}
