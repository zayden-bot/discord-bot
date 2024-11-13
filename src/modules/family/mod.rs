pub mod components;
pub mod slash_commands;

use async_trait::async_trait;
use family::{FamilyManager, FamilyRow};
use serenity::all::{Context, CreateCommand, Ready};
use sqlx::{PgPool, Postgres};
use std::collections::HashMap;
use zayden_core::SlashCommand;

use slash_commands::{
    AdoptCommand, BlockCommand, ChildrenCommand, MarryCommand, ParentsCommand, PartnersCommand,
    RelationshipCommand, SiblingsCommand, TreeCommand, UnblockCommand,
};

use crate::Result;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![
        AdoptCommand::register(ctx, ready)?,
        BlockCommand::register(ctx, ready)?,
        UnblockCommand::register(ctx, ready)?,
        ChildrenCommand::register(ctx, ready)?,
        MarryCommand::register(ctx, ready)?,
        ParentsCommand::register(ctx, ready)?,
        PartnersCommand::register(ctx, ready)?,
        RelationshipCommand::register(ctx, ready)?,
        SiblingsCommand::register(ctx, ready)?,
        TreeCommand::register(ctx, ready)?,
    ];

    Ok(commands)
}

struct FamilyTable;

#[async_trait]
impl FamilyManager<Postgres> for FamilyTable {
    async fn get_row(
        pool: &PgPool,
        user_id: impl Into<i64> + Send,
    ) -> sqlx::Result<Option<FamilyRow>> {
        let user_id: i64 = user_id.into();

        let row = sqlx::query_as!(FamilyRow, "SELECT * FROM family WHERE id = $1", user_id)
            .fetch_optional(pool)
            .await?;

        Ok(row)
    }

    async fn tree<'a>(
        pool: &PgPool,
        user_id: impl Into<i64> + Send,
        mut tree: HashMap<i32, Vec<FamilyRow>>,
        depth: i32,
        add_parents: bool,
        add_partners: bool,
    ) -> sqlx::Result<HashMap<i32, Vec<FamilyRow>>> {
        let user_id: i64 = user_id.into();

        let row = match FamilyTable::get_row(pool, user_id).await? {
            Some(partner) => partner,
            None => FamilyRow::new(user_id, "Unknown".to_string()),
        };

        if tree.entry(depth).or_default().contains(&row) {
            return Ok(tree);
        }

        tree.entry(depth).or_default().push(row.clone());

        for child in &row.children_ids {
            tree = Box::pin(FamilyTable::tree(
                pool,
                *child,
                tree,
                depth + 1,
                false,
                true,
            ))
            .await?;
        }

        if add_partners {
            for partner in &row.partner_ids {
                tree =
                    Box::pin(FamilyTable::tree(pool, *partner, tree, depth, true, false)).await?;
            }
        }

        if add_parents {
            for parent in &row.parent_ids {
                tree = Box::pin(FamilyTable::tree(
                    pool,
                    *parent,
                    tree,
                    depth - 1,
                    true,
                    true,
                ))
                .await?;
            }
        }

        Ok(tree)
    }

    async fn save(pool: &PgPool, row: &FamilyRow) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO family (id, username, partner_ids, parent_ids, children_ids, blocked_ids) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO UPDATE SET username = $2, partner_ids = $3, parent_ids = $4, children_ids = $5, blocked_ids = $6",
            row.id,
            row.username,
            &row.partner_ids,
            &row.parent_ids,
            &row.children_ids,
            &row.blocked_ids
        ).execute(pool).await?;

        Ok(())
    }

    async fn reset(pool: &PgPool) -> sqlx::Result<()> {
        todo!()
    }
}
