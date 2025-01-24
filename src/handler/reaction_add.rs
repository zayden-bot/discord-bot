use serenity::all::{Context, Reaction};
use sqlx::{PgPool, Postgres};

use crate::guilds::college_kings::SUGGESTION_CATEGORY_ID;
use crate::modules::reaction_roles;
use crate::sqlx_lib::GuildTable;
use crate::Result;

use super::Handler;

impl Handler {
    pub(super) async fn reaction_add(
        ctx: &Context,
        reaction: Reaction,
        pool: &PgPool,
    ) -> Result<()> {
        reaction_roles::reaction::reaction_add(ctx, &reaction).await?;

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
