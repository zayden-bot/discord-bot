use serenity::all::{ChannelId, GuildId, MessageId, RoleId};
use serenity::async_trait;
use sqlx::any::AnyQueryResult;
use sqlx::{Database, FromRow, Pool};

#[async_trait]
pub trait ReactionRolesManager<Db: Database> {
    async fn create_row(
        pool: &Pool<Db>,
        guild_id: impl Into<i64> + Send,
        channel_id: impl Into<i64> + Send,
        message_id: impl Into<i64> + Send,
        role_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult>;

    async fn get_guild_rows(
        pool: &Pool<Db>,
        guild_id: impl Into<i64> + Send,
    ) -> sqlx::Result<Vec<ReactionRole>>;

    async fn get_row(
        pool: &Pool<Db>,
        message_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<Option<ReactionRole>>;

    async fn delete_row(
        pool: &Pool<Db>,
        guild_id: impl Into<i64> + Send,
        channel_id: impl Into<i64> + Send,
        message_id: impl Into<i64> + Send,
        emoji: &str,
    ) -> sqlx::Result<AnyQueryResult>;
}

#[derive(FromRow)]
pub struct ReactionRole {
    pub id: i32,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
    pub role_id: i64,
    pub emoji: String,
}

impl ReactionRole {
    pub fn guild_id(&self) -> GuildId {
        GuildId::new(self.guild_id as u64)
    }

    pub fn channel_id(&self) -> ChannelId {
        ChannelId::new(self.channel_id as u64)
    }

    pub fn message_id(&self) -> MessageId {
        MessageId::new(self.message_id as u64)
    }

    pub fn role_id(&self) -> RoleId {
        RoleId::new(self.role_id as u64)
    }
}
