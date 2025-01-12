use serenity::all::{
    ChannelId, Context, CreateEmbed, CreateInteractionResponse, CreateMessage, EditMessage,
    EditThread, ModalInteraction,
};
use zayden_core::parse_modal_data;

use crate::Result;

pub async fn run(ctx: &Context, modal: &ModalInteraction, accepted: bool) -> Result<()> {
    let mut data = parse_modal_data(&modal.data.components);
    let response = data.remove("response").unwrap();

    let mut message = match &modal.message {
        Some(message) => message.clone(),
        _ => unreachable!("Message is required"),
    };

    let old_embed = message.embeds[0].clone();
    let old_url = old_embed.url.expect("URL is required");
    let old_title = old_embed.title.expect("Title is required");

    let channel_id: ChannelId = old_url
        .split('/')
        .nth(5)
        .and_then(|x| x.parse().ok())
        .expect("Invalid URL");

    let prefix = if accepted {
        "[Accepted] - "
    } else {
        "[Rejected] - "
    };
    let name = if old_title.starts_with("[Accepted] - ") || old_title.starts_with("[Rejected] - ") {
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
                    .description(old_embed.description.expect("Description is required"))
                    .field("Team Response", response, false)
                    .author(old_embed.author.expect("Author is required").into())
                    .footer(old_embed.footer.expect("Footer is required").into()),
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

    Ok(())
}
