use serenity::all::{Context, Message};
use sqlx::{PgPool, Postgres};
use ticket::SupportMessageCommand;

use crate::sqlx_lib::GuildTable;
use crate::Result;

pub async fn support(ctx: &Context, msg: &Message, pool: &PgPool) -> Result<()> {
    SupportMessageCommand::run::<Postgres, GuildTable>(ctx, msg, pool).await?;

    Ok(())
}
