use serenity::all::{
    ComponentInteraction, Context, CreateActionRow, CreateInputText, CreateInteractionResponse,
    CreateModal, InputTextStyle,
};

use crate::Result;

pub async fn render_request(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let email_input = CreateInputText::new(InputTextStyle::Short, "Patreon Email", "email")
        .placeholder("example@example.com");

    let character_input = CreateInputText::new(InputTextStyle::Short, "Character", "character")
        .placeholder("Enter the character you want in the render.");

    let prop_input = CreateInputText::new(InputTextStyle::Short, "Prop", "prop")
        .required(false)
        .placeholder("Enter the prop you want in the render.");

    let location_input = CreateInputText::new(InputTextStyle::Short, "Location", "location")
        .required(false)
        .placeholder("Enter the location you want in the render.");

    let render_request =
        CreateInputText::new(InputTextStyle::Paragraph, "Render Request", "description")
            .required(false)
            .placeholder("Describe the render you want, including specific details.");

    let modal = CreateModal::new("render_request", "Render Request").components(vec![
        CreateActionRow::InputText(email_input),
        CreateActionRow::InputText(character_input),
        CreateActionRow::InputText(prop_input),
        CreateActionRow::InputText(location_input),
        CreateActionRow::InputText(render_request),
    ]);

    interaction
        .create_response(&ctx, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}
