use serenity::all::{Context, Member, Reaction};

use crate::{models::ReactionRole, sqlx_lib, Error, Result};

pub async fn reaction_add(ctx: &Context, reaction: &Reaction) -> Result<()> {
    let (reaction_roles, member) = get_reaction_data(ctx, reaction).await?;

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
    let (reaction_roles, member) = get_reaction_data(ctx, reaction).await?;

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

pub async fn get_reaction_data(
    ctx: &Context,
    reaction: &Reaction,
) -> Result<(Vec<ReactionRole>, Member)> {
    let guild_id = reaction.guild_id.ok_or_else(|| Error::NoGuild)?;
    let user_id = reaction.user_id.ok_or_else(|| Error::NoUser)?;

    let member = guild_id.member(&ctx, user_id).await?;
    let reaction_roles = get_reaction_roles(guild_id.get() as i64).await?;

    Ok((reaction_roles, member))
}

pub async fn get_reaction_roles(guild_id: i64) -> Result<Vec<ReactionRole>> {
    let pool = sqlx_lib::get_pool().await?;

    let results = sqlx::query_as!(
        ReactionRole,
        "SELECT * FROM reaction_roles WHERE guild_id = $1",
        guild_id
    )
    .fetch_all(&pool)
    .await?;

    pool.close().await;
    Ok(results)
}
