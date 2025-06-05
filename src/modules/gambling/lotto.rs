use async_trait::async_trait;
use bigdecimal::ToPrimitive;
use gambling::Commands;
use gambling::commands::lotto::{LottoManager, LottoRow};
use gambling::shop::LOTTO_TICKET;
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption, UserId};
use sqlx::any::AnyQueryResult;
use sqlx::{PgConnection, PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct LottoTable;

#[async_trait]
impl LottoManager<Postgres> for LottoTable {
    async fn row(
        conn: &mut PgConnection,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<LottoRow>> {
        let id = id.into();

        sqlx::query_as!(
            LottoRow,
            "SELECT g.id, g.coins, COALESCE(i.quantity, 0) AS quantity FROM gambling g LEFT JOIN gambling_inventory i ON g.id = i.user_id AND i.item_id = $2 WHERE g.id = $1",
            id.get() as i64,
            LOTTO_TICKET.id
        ).fetch_optional(conn).await
    }

    async fn rows(conn: &mut PgConnection) -> sqlx::Result<Vec<LottoRow>> {
        sqlx::query_as!(
            LottoRow,
            "SELECT g.id, g.coins, i.quantity AS quantity FROM gambling g LEFT JOIN gambling_inventory i ON g.id = i.user_id AND i.item_id = $1",
            LOTTO_TICKET.id
        )
        .fetch_all(conn)
        .await
    }

    async fn total_tickets(conn: &mut PgConnection) -> sqlx::Result<i64> {
        sqlx::query_scalar!(
            "SELECT SUM(quantity) FROM gambling_inventory WHERE item_id = $1",
            LOTTO_TICKET.id
        )
        .fetch_one(conn)
        .await
        .map(|x| x.unwrap_or_default())
        .map(|x| x.to_i64().unwrap_or_default())
    }

    async fn delete_tickets(conn: &mut PgConnection) -> sqlx::Result<AnyQueryResult> {
        sqlx::query!(
            "DELETE FROM gambling_inventory WHERE item_id = $1",
            LOTTO_TICKET.id
        )
        .execute(conn)
        .await
        .map(AnyQueryResult::from)
    }

    async fn save(conn: &mut PgConnection, row: LottoRow) -> sqlx::Result<AnyQueryResult> {
        let mut result = sqlx::query!(
            "INSERT INTO gambling (id, coins) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET coins = EXCLUDED.coins;",
            row.id,
            row.coins
        )
        .execute(&mut *conn)
        .await
        .map(AnyQueryResult::from)?;

        let result2 = sqlx::query!(
            "INSERT INTO gambling_inventory (user_id, item_id, quantity) VALUES ($1, $2, $3) ON CONFLICT (user_id, item_id) DO UPDATE SET quantity = EXCLUDED.quantity;",
            row.id,
            LOTTO_TICKET.id,
            row.quantity
        )
        .execute(&mut *conn)
        .await
        .map(AnyQueryResult::from)?;

        result.extend([result2]);

        Ok(result)
    }
}

pub struct Lotto;

#[async_trait]
impl SlashCommand<Error, Postgres> for Lotto {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::lotto::<Postgres, LottoTable>(ctx, interaction, pool).await?;
        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_lotto())
    }
}
