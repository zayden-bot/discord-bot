use async_trait::async_trait;
use gambling::{
    Commands,
    commands::daily::{DailyManager, DailyRow},
};
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption, UserId};
use sqlx::{PgPool, Postgres, any::AnyQueryResult};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct DailyTable;

#[async_trait]
impl DailyManager<Postgres> for DailyTable {
    async fn row(pool: &PgPool, id: impl Into<UserId> + Send) -> sqlx::Result<Option<DailyRow>> {
        let id = id.into();

        sqlx::query_as!(
            DailyRow,
            "SELECT id, coins, daily FROM gambling WHERE id = $1",
            id.get() as i64
        )
        .fetch_optional(pool)
        .await
    }

    async fn save(pool: &PgPool, row: DailyRow) -> sqlx::Result<AnyQueryResult> {
        sqlx::query!(
            "INSERT INTO gambling (id, coins, daily)
            VALUES ($1, $2, now())
            ON CONFLICT (id) DO UPDATE SET
            coins = EXCLUDED.coins, daily = EXCLUDED.daily;",
            row.id,
            row.coins,
        )
        .execute(pool)
        .await
        .map(AnyQueryResult::from)
    }
}

pub struct Daily;

#[async_trait]
impl SlashCommand<Error, Postgres> for Daily {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::daily::<Postgres, DailyTable>(ctx, interaction, pool).await?;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_daily())
    }
}
