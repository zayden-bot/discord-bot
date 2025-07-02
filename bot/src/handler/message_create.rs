use futures::FutureExt;
use serenity::all::{Context, Message};
use sqlx::{PgPool, Postgres};
use zayden_core::MessageCommand;

use crate::Result;
use crate::handler::Handler;
use crate::modules::ai::Ai;
use crate::modules::gambling::GamblingTable;
use crate::modules::levels::LevelsTable;
use crate::modules::ticket::message_commands::support;

impl Handler {
    pub async fn message_create(ctx: &Context, msg: Message, pool: &PgPool) -> Result<()> {
        if msg.author.bot {
            return Ok(());
        }

        let (new_level, ..) = tokio::try_join!(
            levels::message_create::<Postgres, LevelsTable>(&msg, pool).map(Result::Ok),
            Ai::run(ctx, &msg, pool),
            support(ctx, &msg, pool),
        )?;

        if let Some(level) = new_level {
            GamblingTable::add_coins(pool, msg.author.id, level as i64 * 1000)
                .await
                .unwrap();
        }

        Ok(())
    }
}
