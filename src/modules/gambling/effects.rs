use async_trait::async_trait;
use chrono::Utc;
use gambling::{EffectsManager, EffectsRow, shop::ShopItem};
use serenity::all::UserId;
use sqlx::{PgConnection, Postgres, any::AnyQueryResult};

pub struct EffectsTable;

#[async_trait]
impl EffectsManager<Postgres> for EffectsTable {
    async fn get_effects(
        conn: &mut PgConnection,
        user_id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Vec<EffectsRow>> {
        let user_id = user_id.into();

        sqlx::query_as!(
            EffectsRow,
            "SELECT DISTINCT ON (item_id) id, item_id, expiry FROM gambling_effects WHERE user_id = $1",
            user_id.get() as i64,
        )
        .fetch_all(conn)
        .await
    }

    async fn add_effect(
        conn: &mut PgConnection,
        user_id: impl Into<UserId> + Send,
        item: &ShopItem<'_>,
    ) -> sqlx::Result<AnyQueryResult> {
        let user_id = user_id.into();

        let expiry = item
            .effect_duration
            .map(|d| Utc::now() + d)
            .map(|dt| dt.naive_utc());

        sqlx::query!(
            "INSERT INTO gambling_effects (user_id, item_id, expiry) VALUES ($1, $2, $3)",
            user_id.get() as i64,
            item.id,
            expiry,
        )
        .execute(conn)
        .await
        .map(AnyQueryResult::from)
    }

    async fn remove_effect(conn: &mut PgConnection, id: i32) -> sqlx::Result<AnyQueryResult> {
        sqlx::query!("DELETE FROM gambling_effects WHERE id = $1", id)
            .execute(conn)
            .await
            .map(AnyQueryResult::from)
    }
}
