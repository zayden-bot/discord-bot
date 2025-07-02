use serenity::all::{Context, Guild};

use crate::{CachedState, VoiceStateCache};

pub async fn guild_create(ctx: &Context, guild: &Guild) {
    let mut data = ctx.data.write().await;

    for (user_id, state) in guild
        .voice_states
        .iter()
        .filter(|(_, state)| state.channel_id.is_some())
    {
        data.entry::<VoiceStateCache>()
            .and_modify(|cache| {
                cache.insert(
                    *user_id,
                    CachedState::new(state.channel_id, guild.id, state.user_id),
                );
            })
            .or_insert_with(|| {
                [(
                    *user_id,
                    CachedState::new(state.channel_id, guild.id, state.user_id),
                )]
                .into_iter()
                .collect()
            });
    }
}
