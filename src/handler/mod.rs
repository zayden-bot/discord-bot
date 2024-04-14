mod interaction_create;
mod message;
mod reaction;
mod ready;

use serenity::all::{Event, RawEventHandler};
use serenity::async_trait;
use serenity::model::channel::{Message, Reaction};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Interaction;
use serenity::prelude::Context;

use crate::{Result, OSCAR_SIX_ID};

pub struct Handler;

impl Handler {
    async fn message(&self, ctx: &Context, msg: Message) -> Result<()> {
        message::message(ctx, msg).await?;

        Ok(())
    }

    async fn reaction_add(&self, ctx: &Context, reaction: Reaction) -> Result<()> {
        reaction::reaction_add(ctx, reaction).await?;

        Ok(())
    }

    async fn reaction_remove(&self, ctx: &Context, reaction: Reaction) -> Result<()> {
        reaction::reaction_remove(ctx, reaction).await?;

        Ok(())
    }

    async fn ready(&self, ctx: &Context, ready: Ready) -> Result<()> {
        ready::ready(ctx, ready).await?;

        Ok(())
    }

    async fn interaction_create(&self, ctx: &Context, interaction: Interaction) -> Result<()> {
        match &interaction {
            Interaction::Command(command) => {
                interaction_create::interaction_command(ctx, command).await?
            }
            Interaction::Component(component) => {
                interaction_create::interaction_component(ctx, component).await?
            }
            Interaction::Modal(modal) => interaction_create::interaction_modal(ctx, modal).await?,
            _ => unimplemented!("Interaction not implemented: {:?}", interaction.kind()),
        }

        Ok(())
    }
}

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, ctx: Context, ev: Event) {
        let event_name = ev.name().unwrap_or(String::from("Unknown"));
        let ev_debug = format!("{:?}", ev);

        let result = match ev {
            Event::InteractionCreate(interaction) => {
                self.interaction_create(&ctx, interaction.interaction).await
            }
            Event::MessageCreate(msg) => self.message(&ctx, msg.message).await,
            Event::ReactionAdd(reaction) => self.reaction_add(&ctx, reaction.reaction).await,
            Event::ReactionRemove(reaction) => self.reaction_remove(&ctx, reaction.reaction).await,
            Event::Ready(ready) => self.ready(&ctx, ready.ready).await,
            _ => Ok(()),
        };

        if let Err(e) = result {
            let msg = format!("Error handling {:?}: {:?}", event_name, e);
            eprintln!("{}\n{}", msg, ev_debug);

            if let Ok(channel) = OSCAR_SIX_ID.create_dm_channel(&ctx).await {
                let _ = channel.say(&ctx, msg).await;
            }
        }
    }
}
