use serenity::all::{Context, Reaction};
use sqlx::{PgPool, Postgres};
use suggestions::Suggestions;

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
        Suggestions::reaction::<Postgres, GuildTable>(ctx, &reaction, pool).await;

        Ok(())
    }
}
