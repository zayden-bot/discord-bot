use std::collections::HashMap;

use serenity::all::{
    ButtonStyle, CommandInteraction, Context, CreateButton, CreateEmbed, CreateMessage,
    EditInteractionResponse, ResolvedValue,
};

use crate::Result;

use super::TicketCommand;

impl TicketCommand {
    pub(super) async fn create(
        ctx: &Context,
        interaction: &CommandInteraction,
        mut options: HashMap<&str, ResolvedValue<'_>>,
    ) -> Result<()> {
        let Some(ResolvedValue::String(title)) = options.remove("title") else {
            unreachable!("Title is required")
        };

        let Some(ResolvedValue::String(description)) = options.remove("description") else {
            unreachable!("Description is required")
        };

        let Some(ResolvedValue::String(label)) = options.remove("label") else {
            unreachable!("Label is required")
        };

        interaction.defer_ephemeral(ctx).await?;

        let embed = CreateEmbed::new()
            .title(title)
            .description(description.replace("\\n", "\n"));

        let button = CreateButton::new("ticket_create")
            .style(ButtonStyle::Primary)
            .label(label);

        interaction
            .channel_id
            .send_message(ctx, CreateMessage::new().embed(embed).button(button))
            .await
            .unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Ticket embed created"),
            )
            .await
            .unwrap();

        Ok(())
    }
}
