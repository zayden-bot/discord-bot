use async_trait::async_trait;
use gambling::commands::shop::{BuyRow, ListRow, SellRow, ShopManager};
use gambling::{Commands, GamblingItem};
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption, UserId};
use sqlx::types::Json;
use sqlx::{PgPool, Postgres, any::AnyQueryResult};
use zayden_core::SlashCommand;

use crate::modules::gambling::GoalsTable;
use crate::{Error, Result};

pub struct ShopTable;

#[async_trait]
impl ShopManager<Postgres> for ShopTable {
    async fn buy_row(pool: &PgPool, id: impl Into<UserId> + Send) -> sqlx::Result<Option<BuyRow>> {
        let id = id.into();

        sqlx::query_as!(BuyRow,
            r#"SELECT
            g.id,
            g.coins,
            g.gems,
            
            COALESCE(l.level, 0) AS level,

            (
                SELECT jsonb_agg(
                    jsonb_build_object(
                        'quantity', inv.quantity,
                        'item_id', inv.item_id
                    )
                )
                FROM gambling_inventory inv
                WHERE inv.user_id = g.id
            ) as "inventory: Json<Vec<GamblingItem>>",

            m.miners,
            m.mines,
            m.land,
            m.countries,
            m.continents,
            m.planets,
            m.solar_systems,
            m.galaxies,
            m.universes,
            m.prestige,
            m.tech,
            m.utility,
            m.production

            FROM gambling g LEFT JOIN levels l ON g.id = l.id LEFT JOIN gambling_mine m ON g.id = m.id WHERE g.id = $1;"#,
            id.get() as i64
        ).fetch_optional(pool).await
    }

    async fn buy_save(pool: &PgPool, row: BuyRow) -> sqlx::Result<AnyQueryResult> {
        let mut tx = pool.begin().await?;

        let mut result = sqlx::query!(
            "INSERT INTO gambling (id, coins, gems)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET
            coins = EXCLUDED.coins, gems = EXCLUDED.gems;",
            row.id,
            row.coins,
            row.gems,
        )
        .execute(&mut *tx)
        .await
        .map(AnyQueryResult::from)?;

        for item in row.inventory.unwrap_or_default().0 {
            let result2 = sqlx::query!(
                "INSERT INTO gambling_inventory (user_id, item_id, quantity)
                VALUES ($1, $2, $3)
                ON CONFLICT (user_id, item_id) DO UPDATE
                SET quantity = EXCLUDED.quantity",
                row.id,
                item.item_id,
                item.quantity
            )
            .execute(&mut *tx)
            .await
            .map(AnyQueryResult::from)?;

            result.extend([result2]);
        }

        let result3 = sqlx::query!(
            "INSERT INTO gambling_mine (id, miners, mines, land, countries, continents, planets, solar_systems, galaxies, universes, tech, utility, production)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (id) DO UPDATE
            SET
            miners = EXCLUDED.miners,
            mines = EXCLUDED.mines,
            land = EXCLUDED.land,
            countries = EXCLUDED.countries,
            continents = EXCLUDED.continents,
            planets = EXCLUDED.planets,
            solar_systems = EXCLUDED.solar_systems,
            galaxies = EXCLUDED.galaxies,
            universes = EXCLUDED.universes,
            tech = EXCLUDED.tech,
            utility = EXCLUDED.utility,
            production = EXCLUDED.production;",
            row.id,
            row.miners,
            row.mines,
            row.land,
            row.continents,
            row.continents,
            row.planets,
            row.solar_systems,
            row.galaxies,
            row.universes,
            row.tech,
            row.utility,
            row.production,
        ).execute(&mut *tx).await.map(AnyQueryResult::from)?;

        result.extend([result3]);

        tx.commit().await.unwrap();

        Ok(result)
    }

    async fn list_row(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<ListRow>> {
        let id = id.into();

        sqlx::query_as!(
            ListRow,
            r#"SELECT
            g.id,
            g.coins,
            
            (
                SELECT jsonb_agg(
                    jsonb_build_object(
                        'quantity', inv.quantity,
                        'item_id', inv.item_id
                    )
                )
                FROM gambling_inventory inv
                WHERE inv.user_id = g.id
            ) as "inventory: Json<Vec<GamblingItem>>"
            
            FROM gambling g LEFT JOIN levels l ON g.id = l.id WHERE g.id = $1;"#,
            id.get() as i64
        )
        .fetch_optional(pool)
        .await
    }

    async fn sell_row(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<SellRow>> {
        let id = id.into();

        sqlx::query_as!(
            SellRow,
            r#"SELECT
            g.id,
            g.coins,

            (
                SELECT jsonb_agg(
                    jsonb_build_object(
                        'quantity', inv.quantity,
                        'item_id', inv.item_id
                    )
                )
                FROM gambling_inventory inv
                WHERE inv.user_id = g.id
            ) as "inventory: Json<Vec<GamblingItem>>"
            
            FROM gambling g LEFT JOIN levels l ON g.id = l.id WHERE g.id = $1;"#,
            id.get() as i64
        )
        .fetch_optional(pool)
        .await
    }

    async fn sell_save(pool: &PgPool, row: SellRow) -> sqlx::Result<AnyQueryResult> {
        let mut tx = pool.begin().await?;

        let mut result = sqlx::query!(
            "INSERT INTO gambling (id, coins)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET
            coins = EXCLUDED.coins;",
            row.id,
            row.coins,
        )
        .execute(&mut *tx)
        .await
        .map(AnyQueryResult::from)?;

        for item in row.inventory.unwrap_or_default().0 {
            let result2 = sqlx::query!(
                "INSERT INTO gambling_inventory (user_id, item_id, quantity)
                VALUES ($1, $2, $3)
                ON CONFLICT (user_id, item_id) DO UPDATE
                SET quantity = EXCLUDED.quantity",
                row.id,
                item.item_id,
                item.quantity
            )
            .execute(&mut *tx)
            .await
            .map(AnyQueryResult::from)?;

            result.extend([result2]);
        }

        tx.commit().await.unwrap();

        Ok(result)
    }
}

pub struct Shop;

#[async_trait]
impl SlashCommand<Error, Postgres> for Shop {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::shop::<Postgres, GoalsTable, ShopTable>(ctx, interaction, options, pool).await?;
        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_shop())
    }
}
