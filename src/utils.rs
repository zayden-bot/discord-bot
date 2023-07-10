use serenity::builder::CreateEmbed;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{InteractionResponseType, Message};
use serenity::prelude::Context;

pub async fn respond_with_message(ctx: &Context, interaction: &ApplicationCommandInteraction, content: &str) -> Result<(), serenity::Error> {
    interaction.create_interaction_response(&ctx, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource);
        response.interaction_response_data(|message| {
            message.content(content)
        })
    }).await
}

pub async fn respond_with_ephemeral_message(ctx: &Context, interaction: &ApplicationCommandInteraction, content: &str) -> Result<(), serenity::Error> {
    interaction.create_interaction_response(&ctx, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource);
        response.interaction_response_data(|message| {
            message.content(content);
            message.ephemeral(true)
        })
    }).await
}

pub async fn edit_response_with_message(ctx: &Context, interaction: &ApplicationCommandInteraction, content: &str) -> Result<Message, serenity::Error> {
    interaction.edit_original_interaction_response(&ctx, |response| {
        response.content(content);
        response
    }).await
}

pub async fn respond_with_embed<F>(ctx: &Context, interaction: &ApplicationCommandInteraction, embed: F) -> Result<(), serenity::Error>
    where
        F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
{
    interaction.create_interaction_response(&ctx, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource);
        response.interaction_response_data(|message| {
            message.embed(embed);
            message
        })
    }).await
}

pub async fn edit_response_with_embed(ctx: &Context, interaction: &ApplicationCommandInteraction, embed: CreateEmbed) -> Result<Message, serenity::Error> {
    interaction.edit_original_interaction_response(&ctx, |response| {
        response.add_embed(embed);
        response
    }).await
}