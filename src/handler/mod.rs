mod interaction_create;
mod message;
mod reaction;
mod ready;

use serenity::async_trait;
use serenity::model::channel::{Message, Reaction};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Interaction;
use serenity::prelude::{Context, EventHandler};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(e) = message::message(ctx, msg).await {
            println!("Error handling message: {:?}", e);
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Err(e) = reaction::reaction_add(ctx, reaction).await {
            println!("Error handling reaction add: {:?}", e);
        };
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if let Err(e) = reaction::reaction_remove(ctx, reaction).await {
            println!("Error handling reaction remove: {:?}", e);
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Err(e) = ready::ready(ctx, ready).await {
            println!("Error handling ready: {:?}", e);
        };
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let result = match &interaction {
            Interaction::Command(command) => {
                interaction_create::interaction_command(&ctx, command).await
            }
            Interaction::Component(component) => {
                interaction_create::interaction_component(&ctx, component).await
            }
            Interaction::Modal(modal) => interaction_create::interaction_modal(&ctx, modal).await,
            _ => unimplemented!("Interaction not implemented: {:?}", interaction.kind()),
        };

        if let Err(e) = result {
            println!(
                "Error handling interaction create: {:?} | {:?}",
                e, interaction
            );
        };
    }
}
