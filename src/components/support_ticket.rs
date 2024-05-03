use serenity::all::{
    ComponentInteraction, Context, CreateActionRow, CreateInputText, CreateInteractionResponse,
    CreateModal, InputTextStyle,
};

use crate::Result;

pub async fn support_ticket(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let version_input =
        CreateInputText::new(InputTextStyle::Short, "Game Version", "version").placeholder("1.0.0");

    let issue_input = CreateInputText::new(InputTextStyle::Paragraph, "Issue", "issue")
        .placeholder("Describe the issue you're experiencing");

    let additional_input = CreateInputText::new(
        InputTextStyle::Paragraph,
        "Additional Information",
        "additional",
    )
    .required(false)
    .placeholder("Please send a save file that replicates the issue once the ticket is created.");

    let modal = CreateModal::new("support_ticket", "Support Ticket").components(vec![
        CreateActionRow::InputText(version_input),
        CreateActionRow::InputText(issue_input),
        CreateActionRow::InputText(additional_input),
    ]);

    interaction
        .create_response(&ctx, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}
