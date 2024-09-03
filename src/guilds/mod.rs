pub mod college_kings;
pub mod college_kings_team;
pub mod error;

pub use error::ServersTableError;
use serenity::all::{ChannelId, CreateCommand, GuildId, RoleId};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::{Error, Result};

pub fn commands() -> HashMap<GuildId, Vec<CreateCommand>> {
    let mut commands = HashMap::new();
    commands.insert(college_kings::GUILD_ID, college_kings::commands());
    commands.insert(college_kings_team::GUILD_ID, college_kings_team::commands());

    commands
}

pub struct ServersTable;

impl ServersTable {
    pub async fn get_row(pool: &PgPool, guild_id: impl TryInto<i64>) -> Result<Option<ServerRow>> {
        let guild_id: i64 = guild_id.try_into().map_err(|_| Error::ConversionError)?;

        let result = sqlx::query_as!(ServerRow, "SELECT * FROM servers WHERE id = $1", guild_id)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_support_channel_ids(pool: &PgPool) -> Result<Vec<ChannelId>> {
        let result = sqlx::query!(
            "SELECT support_channel_id FROM servers WHERE support_channel_id IS NOT NULL"
        )
        .map(|r| r.support_channel_id)
        .fetch_all(pool)
        .await?;

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
        let id: u64 = self
            .rules_channel_id
            .ok_or(ServersTableError::RulesChannelNotFound)?
            .try_into()?;

        Ok(ChannelId::new(id))
    }

    pub fn get_general_channel_id(&self) -> Result<ChannelId> {
        let id: u64 = self
            .general_channel_id
            .ok_or(ServersTableError::GeneralChannelNotFound)?
            .try_into()?;

        Ok(ChannelId::new(id))
    }

    pub fn get_spoiler_channel_id(&self) -> Result<ChannelId> {
        let id: u64 = self
            .spoiler_channel_id
            .ok_or(ServersTableError::SpoilerChannelNotFound)?
            .try_into()?;

        Ok(ChannelId::new(id))
    }

    pub fn get_support_channel_id(&self) -> Result<ChannelId> {
        let id: u64 = self
            .support_channel_id
            .ok_or(ServersTableError::SupportChannelNotFound)?
            .try_into()?;

        Ok(ChannelId::new(id))
    }

    pub fn get_suggestion_channel_id(&self) -> Result<ChannelId> {
        let id: u64 = self
            .suggestions_channel_id
            .ok_or(ServersTableError::SuggestionsChannelNotFound)?
            .try_into()?;

        Ok(ChannelId::new(id))
    }

    pub fn get_artist_role_id(&self) -> Result<RoleId> {
        let id: u64 = self
            .artist_role_id
            .ok_or(ServersTableError::ArtistRoleNotFound)?
            .try_into()?;

        Ok(RoleId::new(id))
    }

    pub fn get_sleep_role_id(&self) -> Result<RoleId> {
        let id: u64 = self
            .sleep_role_id
            .ok_or(ServersTableError::SleepRoleNotFound)?
            .try_into()?;

        Ok(RoleId::new(id))
    }
}
