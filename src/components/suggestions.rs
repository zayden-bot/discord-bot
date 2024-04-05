use serenity::all::{
    ComponentInteraction, Context, CreateActionRow, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateModal, InputTextStyle, RoleId,
};

use crate::Result;

const TEAM_LEADERS_ROLE_ID: RoleId = RoleId::new(836275726352646176);

pub async fn suggestions(
    ctx: &Context,
    interaction: &ComponentInteraction,
    accepted: bool,
) -> Result<()> {
    if let Some(member) = &interaction.member {
        if !member
            .roles
            .iter()
            .any(|role_id| role_id == &TEAM_LEADERS_ROLE_ID)
        {
            interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .ephemeral(true)
                            .content("You do not have permission to use this component."),
                    ),
                )
                .await?;
            return Ok(());
        }
    }

    let response = CreateInputText::new(InputTextStyle::Paragraph, "Response", "response")
        .required(true)
        .placeholder("Response to the suggestion");

    let id = if accepted {
        "suggestions_accept"
    } else {
        "suggestions_reject"
    };

    let modal = CreateModal::new(id, "Suggestion Response")
        .components(vec![CreateActionRow::InputText(response)]);

    interaction
        .create_response(ctx, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}
