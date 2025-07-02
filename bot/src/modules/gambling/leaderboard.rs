use async_trait::async_trait;
use futures::TryStreamExt;
use gambling::Commands;
use gambling::commands::leaderboard::{
    CoinsRow, EggplantsRow, GemsRow, LeaderboardManager, LeaderboardRow, LottoTicketRow,
    NetworthRow,
};
use gambling::shop::{EGGPLANT, LOTTO_TICKET, WEAPON_CRATE};
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption, UserId};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

const LIMIT: i64 = 10;

pub struct LeaderboardTable;

#[async_trait]
impl LeaderboardManager<Postgres> for LeaderboardTable {
    async fn networth(
        pool: &PgPool,
        users: &[i64],
        page_num: i64,
    ) -> sqlx::Result<Vec<LeaderboardRow>> {
        let offset = (page_num - 1) * LIMIT;

        sqlx::query_as!(
            NetworthRow,
            r#"
            SELECT
                g.id,
                (
                    g.coins +
                    COALESCE(gi_eggplants.quantity, 0) * $3 +
                    COALESCE(gi_crates.quantity, 0) * $5
                ) AS networth
            FROM
                gambling g
            LEFT JOIN
                gambling_inventory gi_eggplants ON g.id = gi_eggplants.user_id
                                            AND gi_eggplants.item_id = $2
            LEFT JOIN
                gambling_inventory gi_crates ON g.id = gi_crates.user_id
                                            AND gi_crates.item_id = $4
            WHERE
                g.id = ANY($1)
            ORDER BY
                networth DESC
            LIMIT $6
            OFFSET $7
            "#,
            users,
            EGGPLANT.id,
            EGGPLANT.coin_cost().unwrap_or_default(),
            WEAPON_CRATE.id,
            WEAPON_CRATE.coin_cost().unwrap_or_default(),
            LIMIT,
            offset
        )
        .fetch(pool)
        .map_ok(LeaderboardRow::NetWorth)
        .try_collect::<Vec<_>>()
        .await
    }

    async fn networth_row_number(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>> {
        let user_id = id.into();

        sqlx::query_scalar!(
            r#"
            WITH user_networths AS (
                SELECT
                    g.id,
                    (
                        g.coins +
                        COALESCE(gi_eggplants.quantity, 0) * $3 +
                        COALESCE(gi_crates.quantity, 0) * $5
                    ) AS networth_value
                FROM
                    gambling g
                LEFT JOIN
                    gambling_inventory gi_eggplants ON g.id = gi_eggplants.user_id
                                                AND gi_eggplants.item_id = $2
                LEFT JOIN
                    gambling_inventory gi_crates ON g.id = gi_crates.user_id
                                                AND gi_crates.item_id = $4
            ),
            ranked_users AS (
                SELECT
                    id,
                    ROW_NUMBER() OVER (ORDER BY networth_value DESC) as rn
                FROM
                    user_networths
            )
            SELECT rn
            FROM ranked_users
            WHERE id = $1 -- User ID ($1)
            "#,
            user_id.get() as i64,
            EGGPLANT.id,
            EGGPLANT.coin_cost().unwrap_or_default(),
            WEAPON_CRATE.id,
            WEAPON_CRATE.coin_cost().unwrap_or_default()
        )
        .fetch_optional(pool)
        .await
        .map(|num_opt_opt| num_opt_opt.flatten())
    }

    async fn coins(
        pool: &PgPool,
        users: &[i64],
        page_num: i64,
    ) -> sqlx::Result<Vec<LeaderboardRow>> {
        let offset = (page_num - 1) * LIMIT;

        sqlx::query_as!(
            CoinsRow,
            r#"
            SELECT id, coins
            FROM gambling
            WHERE id = ANY($1)
            ORDER BY coins DESC
            LIMIT $2
            OFFSET $3
            "#,
            users,
            LIMIT,
            offset
        )
        .fetch(pool)
        .map_ok(LeaderboardRow::Coins)
        .try_collect::<Vec<_>>()
        .await
    }

    async fn coins_row_number(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>> {
        let user_id = id.into();

        sqlx::query_scalar!(
            r#"
        WITH numbered_users AS (
            SELECT
                id,
                ROW_NUMBER() OVER (ORDER BY coins DESC) as rn
            FROM
                gambling
        )
        SELECT rn
        FROM numbered_users
        WHERE id = $1
        "#,
            user_id.get() as i64
        )
        .fetch_optional(pool)
        .await
        .map(|num| num.flatten())
    }

    async fn gems(
        pool: &PgPool,
        users: &[i64],
        page_num: i64,
    ) -> sqlx::Result<Vec<LeaderboardRow>> {
        let offset = (page_num - 1) * LIMIT;

        sqlx::query_as!(
            GemsRow,
            r#"
            SELECT id, gems
            FROM gambling
            WHERE id = ANY($1)
            ORDER BY gems DESC
            LIMIT $2
            OFFSET $3
            "#,
            users,
            LIMIT,
            offset
        )
        .fetch(pool)
        .map_ok(LeaderboardRow::Gems)
        .try_collect::<Vec<_>>()
        .await
    }

    async fn gems_row_number(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>> {
        let user_id = id.into();

        sqlx::query_scalar!(
            r#"
        WITH numbered_users AS (
            SELECT
                id,
                ROW_NUMBER() OVER (ORDER BY gems DESC) as rn
            FROM
                gambling
        )
        SELECT rn
        FROM numbered_users
        WHERE id = $1
        "#,
            user_id.get() as i64
        )
        .fetch_optional(pool)
        .await
        .map(|num| num.flatten())
    }

    async fn eggplants(
        pool: &PgPool,
        users: &[i64],
        page_num: i64,
    ) -> sqlx::Result<Vec<LeaderboardRow>> {
        let offset = (page_num - 1) * LIMIT;

        sqlx::query_as!(
            EggplantsRow,
            r#"
            SELECT user_id, quantity
            FROM gambling_inventory
            WHERE user_id = ANY($1) AND item_id = $2
            ORDER BY quantity DESC
            LIMIT $3
            OFFSET $4
            "#,
            users,
            EGGPLANT.id,
            LIMIT,
            offset
        )
        .fetch(pool)
        .map_ok(LeaderboardRow::Eggplants)
        .try_collect::<Vec<_>>()
        .await
    }

    async fn eggplants_row_number(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>> {
        let id = id.into();

        sqlx::query_scalar!(
            r#"
        WITH RankedUsers AS (
            SELECT
                user_id,
                ROW_NUMBER() OVER (ORDER BY quantity DESC) as row_num
            FROM
                gambling_inventory
            WHERE
                item_id = $1
        )
        SELECT
            row_num
        FROM
            RankedUsers
        WHERE
            user_id = $2;
        "#,
            EGGPLANT.id,
            id.get() as i64
        )
        .fetch_optional(pool)
        .await
        .map(|num| num.flatten())
    }

    async fn lottotickets(pool: &PgPool, page_num: i64) -> sqlx::Result<Vec<LeaderboardRow>> {
        let offset = (page_num - 1) * LIMIT;

        sqlx::query_as!(
            LottoTicketRow,
            r#"
            SELECT user_id, quantity
            FROM gambling_inventory
            WHERE item_id = $1
            ORDER BY quantity DESC
            LIMIT $2
            OFFSET $3
            "#,
            LOTTO_TICKET.id,
            LIMIT,
            offset
        )
        .fetch(pool)
        .map_ok(LeaderboardRow::LottoTickets)
        .try_collect::<Vec<_>>()
        .await
    }

    async fn lottotickets_row_number(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>> {
        let id = id.into();

        sqlx::query_scalar!(
            r#"
        WITH RankedUsers AS (
            SELECT
                user_id,
                ROW_NUMBER() OVER (ORDER BY quantity DESC) as row_num
            FROM
                gambling_inventory
            WHERE
                item_id = $1
        )
        SELECT
            row_num
        FROM
            RankedUsers
        WHERE
            user_id = $2;
        "#,
            LOTTO_TICKET.id,
            id.get() as i64
        )
        .fetch_optional(pool)
        .await
        .map(|num| num.flatten())
    }
}

pub struct Leaderboard;

#[async_trait]
impl SlashCommand<Error, Postgres> for Leaderboard {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::leaderboard::<Postgres, LeaderboardTable>(ctx, interaction, options, pool)
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_leaderboard())
    }
}
