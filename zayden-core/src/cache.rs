use std::collections::HashMap;

use serenity::all::{Context, Guild, GuildId, UserId};
use serenity::prelude::TypeMapKey;

pub struct GuildMembersCache;

impl GuildMembersCache {
    pub async fn guild_create(ctx: &Context, guild: &Guild) {
        let mut data = ctx.data.write().await;

        data.entry::<GuildMembersCache>()
            .or_insert_with(HashMap::new)
            .insert(guild.id, guild.members.keys().copied().collect());
    }
}

impl TypeMapKey for GuildMembersCache {
    type Value = HashMap<GuildId, Vec<UserId>>;
}
