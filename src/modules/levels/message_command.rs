use async_trait::async_trait;
use chrono::{TimeDelta, Utc};
use serenity::all::{Context, Message};
use sqlx::{PgPool, Postgres};
use zayden_core::MessageCommand;

use crate::modules::gambling::GamblingTable;
use crate::{Error, Result};

use super::{Levels, LevelsRow};

#[async_trait]
impl MessageCommand<Error, Postgres> for Levels {
    async fn run(_ctx: &Context, message: &Message, pool: &PgPool) -> Result<()> {
        if message.guild_id.is_none() {
            return Ok(());
        }

        let mut row = LevelsRow::from_table(pool, message.author.id)
            .await
            .unwrap();

        let xp_cooldown = row.last_xp + TimeDelta::minutes(1);

        if xp_cooldown > Utc::now().naive_utc() {
            return Ok(());
        }

        if row.update_level() {
            GamblingTable::add_cash(pool, message.author.id, (row.level * 1000) as i64)
                .await
                .unwrap();
        }

        row.save(pool).await.unwrap();

        Ok(())
    }
}
