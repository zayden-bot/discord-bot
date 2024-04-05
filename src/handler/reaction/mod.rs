mod reaction_roles;
mod suggestions;

use serenity::all::{Context, Reaction};

use crate::{college_kings::SUGGESTION_CATEGORY_ID, Result};

pub async fn reaction_add(ctx: Context, reaction: Reaction) -> Result<()> {
    reaction_roles::reaction_add(&ctx, &reaction).await?;

    if let Some(channel) = reaction.channel(&ctx).await?.guild() {
        if channel.parent_id == Some(SUGGESTION_CATEGORY_ID) {
            suggestions::suggestion(&ctx, &reaction, channel).await?;
        }
    }

    Ok(())
}

pub async fn reaction_remove(ctx: Context, reaction: Reaction) -> Result<()> {
    reaction_roles::reaction_remove(&ctx, &reaction).await?;

    if let Some(channel) = reaction.channel(&ctx).await?.guild() {
        if channel.parent_id == Some(SUGGESTION_CATEGORY_ID) {
            suggestions::suggestion(&ctx, &reaction, channel).await?;
        }
    }

    Ok(())
}
