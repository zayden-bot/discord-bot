use async_trait::async_trait;
use gambling::commands::gift::GiftManager;
use gambling::{Commands, commands::gift::SenderRow};
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption, UserId};
use sqlx::{PgPool, Postgres, any::AnyQueryResult};
use zayden_core::SlashCommand;

use crate::modules::gambling::GoalsTable;
use crate::{Error, Result};

pub struct GiftTable;

#[async_trait]
impl GiftManager<Postgres> for GiftTable {
    async fn sender(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<SenderRow>> {
        let id = id.into();

        sqlx::query_as!(
            SenderRow,
            "SELECT g.id, g.coins, g.gems, g.gift, COALESCE(l.level, 0) AS level FROM gambling g LEFT JOIN levels l ON g.id = l.id WHERE g.id = $1",
            id.get() as i64
        )
        .fetch_optional(pool)
        .await
    }

    async fn add_coins(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
        amount: i64,
    ) -> sqlx::Result<AnyQueryResult> {
        let id = id.into();

        sqlx::query!(
            "UPDATE gambling SET coins = coins + $2 WHERE id = $1",
            id.get() as i64,
            amount
        )
        .execute(pool)
        .await
        .map(AnyQueryResult::from)
    }

    async fn save_sender(pool: &PgPool, row: SenderRow) -> sqlx::Result<AnyQueryResult> {
        let mut tx = pool.begin().await?;

        let mut result = sqlx::query!(
            "INSERT INTO gambling (id, coins, gems, gift)
            VALUES ($1, $2, $3, now())
            ON CONFLICT (id) DO UPDATE SET
            coins = EXCLUDED.coins, gems = EXCLUDED.gems, gift = EXCLUDED.gift;",
            row.id,
            row.coins,
            row.gems,
        )
        .execute(&mut *tx)
        .await
        .map(AnyQueryResult::from)?;

        let result2 = sqlx::query!(
            "INSERT INTO levels (id, level)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET
            level = EXCLUDED.level;",
            row.id,
            row.level,
        )
        .execute(&mut *tx)
        .await
        .map(AnyQueryResult::from)?;

        result.extend([result2]);

        tx.commit().await?;

        Ok(result)
    }
}

pub struct Gift;

#[async_trait]
impl SlashCommand<Error, Postgres> for Gift {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::gift::<Postgres, GoalsTable, GiftTable>(ctx, interaction, options, pool).await?;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_gift())
    }
}
