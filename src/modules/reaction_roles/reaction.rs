use reaction_roles::ReactionRoleReaction;
use serenity::all::{Context, Reaction};
use sqlx::Postgres;

use crate::Result;
use crate::sqlx_lib::PostgresPool;

use super::ReactionRolesTable;

pub async fn reaction_add(ctx: &Context, reaction: &Reaction) -> Result<()> {
    let pool = PostgresPool::get(ctx).await;

    ReactionRoleReaction::reaction_add::<Postgres, ReactionRolesTable>(ctx, reaction, &pool)
        .await?;

    Ok(())
}

pub async fn reaction_remove(ctx: &Context, reaction: &Reaction) -> Result<()> {
    let pool = PostgresPool::get(ctx).await;

    ReactionRoleReaction::reaction_remove::<Postgres, ReactionRolesTable>(ctx, reaction, &pool)
        .await?;

    Ok(())
}
