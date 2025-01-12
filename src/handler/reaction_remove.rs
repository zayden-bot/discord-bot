use serenity::all::{Context, Reaction};

use crate::guilds::college_kings::SUGGESTION_CATEGORY_ID;
use crate::modules::reaction_roles;
use crate::Result;

use super::{suggestions, Handler};

impl Handler {
    pub(super) async fn reaction_remove(ctx: &Context, reaction: Reaction) -> Result<()> {
        reaction_roles::reaction::reaction_remove(ctx, &reaction).await?;

        if let Some(channel) = reaction.channel(&ctx).await.unwrap().guild() {
            if channel.parent_id == Some(SUGGESTION_CATEGORY_ID) {
                suggestions::suggestion(ctx, &reaction, channel).await?;
            }
        }

        Ok(())
    }
}
