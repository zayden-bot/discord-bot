pub mod college_kings;
pub mod college_kings_team;

use serenity::all::{ChannelId, Context, CreateCommand, GuildId, Ready, RoleId};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::Result;

pub fn commands(ctx: &Context, ready: &Ready) -> Result<HashMap<GuildId, Vec<CreateCommand>>> {
    let mut commands = HashMap::new();
    commands.insert(
        college_kings::GUILD_ID,
        college_kings::commands(ctx, ready)?,
    );
    commands.insert(
        college_kings_team::GUILD_ID,
        college_kings_team::commands(ctx, ready)?,
    );

    Ok(commands)
}

pub struct ServersTable;

impl ServersTable {
    pub async fn get_row(pool: &PgPool, id: GuildId) -> Result<Option<ServerRow>> {
        let guild_id: i64 = id.get() as i64;

        let result = sqlx::query_as!(ServerRow, "SELECT * FROM servers WHERE id = $1", guild_id)
            .fetch_optional(pool)
            .await
            .unwrap();

        Ok(result)
    }

    pub async fn get_support_channel_ids(pool: &PgPool) -> Result<Vec<ChannelId>> {
        let result = sqlx::query!(
            "SELECT support_channel_id FROM servers WHERE support_channel_id IS NOT NULL"
        )
        .map(|r| r.support_channel_id)
        .fetch_all(pool)
        .await
        .unwrap();

        let channel_ids = result
            .into_iter()
            .flatten()
            .map(|id| ChannelId::new(id as u64))
            .collect();

        Ok(channel_ids)
    }
}
pub struct ServerRow {
    pub id: i64,
    pub rules_channel_id: Option<i64>,
    pub general_channel_id: Option<i64>,
    pub spoiler_channel_id: Option<i64>,
    pub support_channel_id: Option<i64>,
    pub suggestions_channel_id: Option<i64>,
    pub support_role_id: Option<i64>,
    pub support_thread_id: i32,
    pub artist_role_id: Option<i64>,
    pub sleep_role_id: Option<i64>,
}

impl ServerRow {
    pub fn get_rules_channel_id(&self) -> Result<ChannelId> {
        let id = self.rules_channel_id.unwrap();

        Ok(ChannelId::new(id as u64))
    }

    pub fn get_general_channel_id(&self) -> Result<ChannelId> {
        let id = self.general_channel_id.unwrap();

        Ok(ChannelId::new(id as u64))
    }

    pub fn get_spoiler_channel_id(&self) -> Result<ChannelId> {
        let id = self.spoiler_channel_id.unwrap();

        Ok(ChannelId::new(id as u64))
    }

    pub fn get_support_channel_id(&self) -> Result<ChannelId> {
        let id = self.support_channel_id.unwrap();

        Ok(ChannelId::new(id as u64))
    }

    pub fn get_suggestion_channel_id(&self) -> Result<ChannelId> {
        let id = self.suggestions_channel_id.unwrap();

        Ok(ChannelId::new(id as u64))
    }

    pub fn get_artist_role_id(&self) -> Result<RoleId> {
        let id = self.artist_role_id.unwrap();

        Ok(RoleId::new(id as u64))
    }

    pub fn sleep_role_id(&self) -> Option<RoleId> {
        self.sleep_role_id.map(|id| RoleId::new(id as u64))
    }
}
