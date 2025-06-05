use async_trait::async_trait;
use gambling::Commands;
use gambling::commands::work::{WorkManager, WorkRow};
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption, UserId};
use sqlx::any::AnyQueryResult;
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::modules::gambling::StaminaTable;
use crate::{Error, Result};

use super::goals::GoalsTable;

pub struct WorkTable;

#[async_trait]
impl WorkManager<Postgres> for WorkTable {
    async fn row(pool: &PgPool, id: impl Into<UserId> + Send) -> sqlx::Result<Option<WorkRow>> {
        let id = id.into();

        sqlx::query_as!(
            WorkRow,
            "SELECT g.id, g.coins, g.gems, g.stamina, COALESCE(l.level, 0) AS level FROM gambling g LEFT JOIN levels l ON g.id = l.id WHERE g.id = $1;",
            id.get() as i64
        ).fetch_optional(pool).await
    }

    async fn save(pool: &PgPool, row: WorkRow) -> sqlx::Result<AnyQueryResult> {
        sqlx::query!(
            "INSERT INTO gambling (id, coins, gems, stamina)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
            coins = EXCLUDED.coins, gems = EXCLUDED.gems, stamina = EXCLUDED.stamina;",
            row.id,
            row.coins,
            row.gems,
            row.stamina
        )
        .execute(pool)
        .await
        .map(AnyQueryResult::from)
    }
}

pub struct Work;

#[async_trait]
impl SlashCommand<Error, Postgres> for Work {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::work::<Postgres, StaminaTable, GoalsTable, WorkTable>(ctx, interaction, pool)
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_work())
    }
}
