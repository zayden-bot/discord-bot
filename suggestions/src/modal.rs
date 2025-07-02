use serenity::all::{
    ActionRowComponent, ChannelId, Context, CreateEmbed, CreateInteractionResponse, CreateMessage,
    EditMessage, EditThread, ModalInteraction,
};

use crate::Suggestions;

impl Suggestions {
    pub async fn modal(ctx: &Context, modal: &ModalInteraction, accepted: bool) {
        let ActionRowComponent::InputText(text) = &modal
            .data
            .components
            .first()
            .unwrap()
            .components
            .first()
            .unwrap()
        else {
            unreachable!("InputText is required");
        };

        let response = text.value.as_deref().unwrap();

        let Some(mut message) = modal.message.clone() else {
            unreachable!("Message is required");
        };

        let old_embed = message.embeds.pop().unwrap();
        let old_url = old_embed.url.unwrap();
        let old_title = old_embed.title.unwrap();

        let channel_id = old_url
            .split('/')
            .nth(5)
            .unwrap()
            .parse::<ChannelId>()
            .unwrap();

        let prefix = if accepted {
            "[Accepted] - "
        } else {
            "[Rejected] - "
        };

        let name =
            if old_title.starts_with("[Accepted] - ") || old_title.starts_with("[Rejected] - ") {
                format!("{}{}", prefix, &old_title[11..])
            } else {
                format!("{}{}", prefix, old_title)
                    .chars()
                    .take(100)
                    .collect::<String>()
            };

        channel_id
            .edit_thread(ctx, EditThread::new().name(&name).archived(false))
            .await
            .unwrap();

        message
            .edit(
                ctx,
                EditMessage::new().embed(
                    CreateEmbed::new()
                        .title(name)
                        .url(&old_url)
                        .description(old_embed.description.unwrap())
                        .field("Team Response", response, false)
                        .author(old_embed.author.unwrap().into())
                        .footer(old_embed.footer.unwrap().into()),
                ),
            )
            .await
            .unwrap();

        modal
            .create_response(ctx, CreateInteractionResponse::Acknowledge)
            .await
            .unwrap();

        let title = if accepted {
            "Suggestion Accepted"
        } else {
            "Suggestion Rejected"
        };

        channel_id
            .send_message(
                ctx,
                CreateMessage::new().embed(CreateEmbed::new().title(title).description(response)),
            )
            .await
            .unwrap()
            .pin(ctx)
            .await
            .unwrap();
    }
}
