use serenity::all::{Context, Reaction};
use sqlx::{PgPool, Postgres};

use crate::modules::reaction_roles;
use crate::Result;
use crate::{guilds::college_kings::SUGGESTION_CATEGORY_ID, sqlx_lib::GuildTable};

use super::Handler;

impl Handler {
    pub(super) async fn reaction_remove(
        ctx: &Context,
        reaction: Reaction,
        pool: &PgPool,
    ) -> Result<()> {
        reaction_roles::reaction::reaction_remove(ctx, &reaction).await?;

        if let Some(channel) = reaction.channel(&ctx).await.unwrap().guild() {
            if channel.parent_id == Some(SUGGESTION_CATEGORY_ID) {
                suggestions::Suggestions::reaction_add::<Postgres, GuildTable>(
                    ctx, &reaction, pool, channel,
                )
                .await;
            }
        }

        Ok(())
    }
}
