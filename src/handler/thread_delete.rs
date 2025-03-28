use serenity::all::{Context, PartialGuildChannel};
use sqlx::{PgPool, Postgres};

use crate::Result;
use crate::modules::destiny2::lfg::LfgPostTable;

use super::Handler;

impl Handler {
    pub async fn thread_delete(
        _ctx: &Context,
        thread: PartialGuildChannel,
        pool: &PgPool,
    ) -> Result<()> {
        lfg::events::thread_delete::<Postgres, LfgPostTable>(&thread, pool).await;

        Ok(())
    }
}
