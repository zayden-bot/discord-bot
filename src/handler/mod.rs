use serenity::all::{Event, InteractionCreateEvent, RawEventHandler};
use serenity::async_trait;
use serenity::model::prelude::Interaction;
use serenity::prelude::Context;

use crate::SUPER_USERS;
use crate::sqlx_lib::PostgresPool;

mod guild_create;
mod interaction;
mod message;
mod reaction_add;
mod reaction_remove;
mod ready;
mod thread_delete;
mod voice_state_update;

pub struct Handler;

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, ctx: Context, ev: Event) {
        let event_name = ev.name().unwrap_or(String::from("Unknown"));
        let ev_command_name = match &ev {
            Event::InteractionCreate(InteractionCreateEvent {
                interaction: Interaction::Command(interaction),
                ..
            }) => interaction.data.name.clone(),
            _ => String::from("Unknown"),
        };
        let ev_debug = format!("{:?}", ev);

        let pool = PostgresPool::get(&ctx).await;

        let result = match ev {
            Event::GuildCreate(event) => Self::guild_create(&ctx, event.guild, &pool).await,
            Event::MessageCreate(event) => Self::message_create(&ctx, event.message, &pool).await,
            Event::ReactionAdd(event) => Self::reaction_add(&ctx, event.reaction, &pool).await,
            Event::ReactionRemove(event) => {
                Self::reaction_remove(&ctx, event.reaction, &pool).await
            }
            Event::Ready(event) => Self::ready(&ctx, event.ready, &pool).await,
            Event::VoiceStateUpdate(event) => {
                Self::voice_state_update(&ctx, event.voice_state, &pool).await
            }
            Event::InteractionCreate(event) => {
                Self::interaction_create(&ctx, event.interaction, &pool).await
            }
            Event::ThreadDelete(event) => Self::thread_delete(&ctx, event.thread, &pool).await,
            _ => Ok(()),
        };

        if let Err(e) = result {
            let msg = format!("Error handling {event_name} | {ev_command_name}: {:?}", e);
            eprintln!("\n{}\n{}\n", msg, ev_debug);

            for user in SUPER_USERS {
                if let Ok(channel) = user.create_dm_channel(&ctx).await {
                    let _ = channel.say(&ctx, &msg).await;
                }
            }
        }
    }
}
