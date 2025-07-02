use serenity::all::{
    ComponentInteraction, Context, CreateActionRow, CreateInputText, CreateInteractionResponse,
    CreateModal, InputTextStyle,
};

use crate::Suggestions;

impl Suggestions {
    pub async fn components(ctx: &Context, interaction: &ComponentInteraction, accepted: bool) {
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
    }
}
