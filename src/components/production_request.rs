use serenity::all::{
    ComponentInteraction, Context, CreateActionRow, CreateInputText, CreateInteractionResponse,
    CreateModal, InputTextStyle,
};

use crate::Result;

pub async fn production_request(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let app_name_input =
        CreateInputText::new(InputTextStyle::Short, "App Name", "app_name").value("CK2");

    let episode_input =
        CreateInputText::new(InputTextStyle::Short, "Episode", "episode").placeholder("Ep4");

    let scene_input =
        CreateInputText::new(InputTextStyle::Short, "Scene", "scene").placeholder("S69");

    let request_input = CreateInputText::new(InputTextStyle::Paragraph, "Request", "request")
        .placeholder(
            "Please provide a detailed description of the request you would like to make.",
        );

    let affected_teams = CreateInputText::new(InputTextStyle::Short, "Affected Teams", "teams")
        .required(false)
        .placeholder("Narrative, Programming, Transcribing, Art");

    let modal = CreateModal::new("production_request", "Production Request").components(vec![
        CreateActionRow::InputText(app_name_input),
        CreateActionRow::InputText(episode_input),
        CreateActionRow::InputText(scene_input),
        CreateActionRow::InputText(request_input),
        CreateActionRow::InputText(affected_teams),
    ]);

    interaction
        .create_response(&ctx, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}
