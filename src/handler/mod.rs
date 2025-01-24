use serenity::all::{Event, InteractionCreateEvent, RawEventHandler};
use serenity::async_trait;
use serenity::model::prelude::Interaction;
use serenity::prelude::Context;

pub use ready::OnReady;

use crate::sqlx_lib::PostgresPool;
use crate::SUPER_USERS;

mod interaction;
mod message;
mod reaction_add;
mod reaction_remove;
mod ready;

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
            Event::InteractionCreate(interaction) => {
                Self::interaction_create(&ctx, interaction.interaction, &pool).await
            }
            Event::MessageCreate(msg) => Self::message(&ctx, msg.message, &pool).await,
            Event::ReactionAdd(reaction) => {
                Self::reaction_add(&ctx, reaction.reaction, &pool).await
            }
            Event::ReactionRemove(reaction) => {
                Self::reaction_remove(&ctx, reaction.reaction, &pool).await
            }
            Event::Ready(ready) => Self::ready(&ctx, ready.ready).await,
            _ => Ok(()),
        };

        if let Err(e) = result {
            let msg = format!("Error handling {event_name} | {ev_command_name}: {:?}", e);
            eprintln!("\n{}\n{}\n", msg, ev_debug);

            for user in SUPER_USERS {
                let channel = user.create_dm_channel(&ctx).await.unwrap();
                channel.say(&ctx, &msg).await.unwrap();
            }
        }
    }
}
