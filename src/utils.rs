use serenity::all::{
    CommandInteraction, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, EditInteractionResponse, Message,
};

pub async fn respond_with_message(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: &str,
) -> Result<(), serenity::Error> {
    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(content),
            ),
        )
        .await
}

pub async fn respond_with_ephemeral_message(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: &str,
) -> Result<(), serenity::Error> {
    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(content)
                    .ephemeral(true),
            ),
        )
        .await
}

pub async fn edit_response_with_message(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: &str,
) -> Result<Message, serenity::Error> {
    interaction
        .edit_response(ctx, EditInteractionResponse::new().content(content))
        .await
}

pub async fn respond_with_embed(
    ctx: &Context,
    interaction: &CommandInteraction,
    embed: CreateEmbed,
) -> Result<(), serenity::Error> {
    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().add_embed(embed),
            ),
        )
        .await
}

#[allow(dead_code)]
pub async fn edit_response_with_embed(
    ctx: &Context,
    interaction: &CommandInteraction,
    embed: CreateEmbed,
) -> Result<Message, serenity::Error> {
    interaction
        .edit_response(ctx, EditInteractionResponse::new().add_embed(embed))
        .await
}
