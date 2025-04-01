use serenity::all::{Context, MessageDeleteEvent};
use sqlx::{PgPool, Postgres};

use crate::Result;
use crate::modules::destiny2::lfg::LfgMessageTable;

use super::Handler;

impl Handler {
    pub async fn message_delete(
        _ctx: &Context,
        event: MessageDeleteEvent,
        pool: &PgPool,
    ) -> Result<()> {
        lfg::events::message_delete::<Postgres, LfgMessageTable>(&event, pool).await;

        Ok(())
    }
}
