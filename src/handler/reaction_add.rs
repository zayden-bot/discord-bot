use serenity::all::{Context, Reaction};

use crate::guilds::college_kings::SUGGESTION_CATEGORY_ID;
use crate::modules::reaction_roles;
use crate::Result;

use super::{Handler, suggestions};

impl Handler {
    pub(super) async fn reaction_add(ctx: &Context, reaction: Reaction) -> Result<()> {
        reaction_roles::reaction::reaction_add(ctx, &reaction).await?;

        if let Some(channel) = reaction.channel(&ctx).await?.guild() {
            if channel.parent_id == Some(SUGGESTION_CATEGORY_ID) {
                suggestions::suggestion(ctx, &reaction, channel).await?;
            }
        }

        Ok(())
    }
}
