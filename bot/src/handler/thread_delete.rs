use serenity::all::{Context, PartialGuildChannel};
use sqlx::{PgPool, Postgres};

use crate::Result;
use crate::modules::lfg::PostTable;

use super::Handler;

impl Handler {
    pub async fn thread_delete(
        ctx: &Context,
        thread: PartialGuildChannel,
        pool: &PgPool,
    ) -> Result<()> {
        lfg::events::thread_delete::<Postgres, PostTable>(ctx, &thread, pool).await;

        Ok(())
    }
}
