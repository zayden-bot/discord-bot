use reqwest::Client;
use serenity::all::{
    ComponentInteraction, Context, CreateActionRow, CreateInputText, CreateInteractionResponse,
    CreateModal, InputTextStyle,
};

use crate::{patreon_lib, Result};

pub async fn render_request(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
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

    let mut components = Vec::with_capacity(5);

    let result = patreon_lib::get_user(&Client::new(), interaction.user.id, false).await;
    if result.is_ok_and(|member| member.tier.amount_cents < 5000) {
        let email_input = CreateInputText::new(InputTextStyle::Short, "Patreon Email", "email")
            .placeholder("example@example.com");
        components.push(CreateActionRow::InputText(email_input))
    };

    components.extend([
        CreateActionRow::InputText(character_input),
        CreateActionRow::InputText(prop_input),
        CreateActionRow::InputText(location_input),
        CreateActionRow::InputText(render_request),
    ]);

    let modal = CreateModal::new("render_request", "Render Request").components(components);

    interaction
        .create_response(&ctx, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}
