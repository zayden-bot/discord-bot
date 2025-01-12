use serenity::all::{
    ComponentInteraction, Context, CreateActionRow, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateModal, InputTextStyle,
};

use crate::{guilds::college_kings_team::TEAM_LEADERS_ROLE_ID, Result};

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
                .await
                .unwrap();
            return Ok(());
        }
    }

    let response = CreateInputText::new(InputTextStyle::Paragraph, "Response", "response")
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
        .await
        .unwrap();

    Ok(())
}
