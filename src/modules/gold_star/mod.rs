pub mod slash_commands;

use async_trait::async_trait;
use gold_star::manager::{GoldStarManager, GoldStarRow};
use serenity::all::{Context, CreateCommand, Ready};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use slash_commands::{GiveStarCommand, StarsCommand};

use crate::Result;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![
        GiveStarCommand::register(ctx, ready)?,
        StarsCommand::register(ctx, ready)?,
    ];

    Ok(commands)
}

struct GoldStarTable;

#[async_trait]
impl GoldStarManager<Postgres> for GoldStarTable {
    async fn get_row(
        pool: &PgPool,
        user_id: impl Into<i64> + Send,
    ) -> sqlx::Result<Option<GoldStarRow>> {
        let row = sqlx::query_as!(
            GoldStarRow,
            "SELECT * FROM gold_stars WHERE id = $1",
            user_id.into()
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    async fn save_row(pool: &PgPool, row: &GoldStarRow) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO gold_stars (id, number_of_stars, given_stars, received_stars, last_free_star) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET number_of_stars = $2, given_stars = $3, received_stars = $4, last_free_star = $5", row.id, row.number_of_stars, row.given_stars, row.received_stars, row.last_free_star)
            .execute(pool)
            .await?;

        Ok(())
    }
}
