use async_trait::async_trait;
use gambling::commands::inventory::{InventoryManager, InventoryRow};
use gambling::{Commands, GamblingItem};
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption, UserId};
use sqlx::types::Json;
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::modules::gambling::EffectsTable;
use crate::{Error, Result};

pub struct InventoryTable;

// pub coins: i64,
// pub gems: i64,
// pub inventory: Option<Json<Vec<GamblingItem>>>,
// pub tech: i64,
// pub utility: i64,
// pub production: i64,
// pub coal: i64,
// pub iron: i64,
// pub gold: i64,
// pub redstone: i64,
// pub lapis: i64,
// pub diamonds: i64,
// pub emeralds: i64,

#[async_trait]
impl InventoryManager<Postgres> for InventoryTable {
    async fn row(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<InventoryRow>> {
        let id = id.into();

        sqlx::query_as!(
            InventoryRow,
            r#"SELECT
            g.coins,
            g.gems,

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

            m.tech,
            m.utility,
            m.production,
            m.coal,
            m.iron,
            m.gold,
            m.redstone,
            m.lapis,
            m.diamonds,
            m.emeralds

            FROM gambling g LEFT JOIN gambling_mine m ON g.id = m.id WHERE g.id = $1;"#,
            id.get() as i64
        )
        .fetch_optional(pool)
        .await
    }

    async fn edit_item_quantity(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
        item_id: &str,
        amount: i64,
    ) -> sqlx::Result<i64> {
        let id = id.into();

        sqlx::query_scalar!(
            r#"
        WITH updated_row AS (
            UPDATE gambling_inventory
            SET quantity = quantity - $3
            WHERE user_id = $1
              AND item_id = $2
              AND $3 <= gambling_inventory.quantity
            RETURNING quantity
        ),
        deleted_row AS (
            DELETE FROM gambling_inventory
            WHERE user_id = $1 AND item_id = $2
            AND EXISTS (SELECT 1 FROM updated_row ur WHERE ur.quantity <= 0)
            RETURNING item_id
        )
        SELECT
            ur.quantity
        FROM
            updated_row ur
        "#,
            id.get() as i64,
            item_id,
            amount
        )
        .fetch_one(pool)
        .await
    }
}

pub struct Inventory;

#[async_trait]
impl SlashCommand<Error, Postgres> for Inventory {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::inventory::<Postgres, EffectsTable, InventoryTable>(
            ctx,
            interaction,
            options,
            pool,
        )
        .await?;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_inventory())
    }
}
