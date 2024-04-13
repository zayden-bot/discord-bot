use serenity::all::{Context, Reaction};
use sqlx::{Pool, Postgres};

use crate::{models::ReactionRole, sqlx_lib::PostgresPool, Error, Result};

pub async fn reaction_add(ctx: &Context, reaction: &Reaction) -> Result<()> {
    let guild_id = reaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let data = ctx.data.read().await;
    let pool = data
        .get::<PostgresPool>()
        .expect("PostgresPool should exist in data.");

    let reaction_roles = get_reaction_roles(pool, guild_id).await?;
    let member = reaction.member.as_ref().ok_or_else(|| Error::NoMember)?;

    for reaction_role in reaction_roles {
        if (reaction.message_id.get() == (reaction_role.message_id as u64))
            && (reaction.emoji.to_string() == reaction_role.emoji)
        {
            member.add_role(&ctx, reaction_role.role_id as u64).await?;
        }
    }

    Ok(())
}

pub async fn reaction_remove(ctx: &Context, reaction: &Reaction) -> Result<()> {
    let guild_id = reaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let data = ctx.data.read().await;
    let pool = data
        .get::<PostgresPool>()
        .expect("PostgresPool should exist in data.");

    let reaction_roles = get_reaction_roles(pool, guild_id).await?;
    let member = match reaction.member {
        Some(ref member) => member.clone(),
        None => {
            guild_id
                .member(ctx, reaction.user_id.ok_or_else(|| Error::NoUser)?)
                .await?
        }
    };

    for reaction_role in reaction_roles {
        if (reaction.message_id.get() == (reaction_role.message_id as u64))
            && (reaction.emoji.to_string() == reaction_role.emoji)
        {
            member
                .remove_role(&ctx, reaction_role.role_id as u64)
                .await?;
        }
    }

    Ok(())
}

pub async fn get_reaction_roles(
    pool: &Pool<Postgres>,
    guild_id: impl TryInto<i64>,
) -> Result<Vec<ReactionRole>> {
    let reaction_roles = sqlx::query_as!(
        ReactionRole,
        "SELECT * FROM reaction_roles WHERE guild_id = $1",
        guild_id.try_into().map_err(|_| Error::ConversionError)?
    )
    .fetch_all(pool)
    .await?;

    Ok(reaction_roles)
}
