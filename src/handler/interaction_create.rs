use serenity::all::{
    CommandInteraction, ComponentInteraction, Context, CreateActionRow, CreateInputText,
    CreateInteractionResponse, CreateModal, InputTextStyle, ModalInteraction,
};

use crate::{commands::slash_commands::*, modals, Result};

pub async fn interaction_command(ctx: &Context, command: CommandInteraction) -> Result<()> {
    let command_name = &command.data.name;

    println!("{} ran command: {}", command.user.tag(), command_name);

    match command_name.as_str() {
        "add_artist" => add_artist::run(ctx, &command).await?,
        "close" => close::run(ctx, &command).await?,
        "fetch_suggestions" => fetch_suggestions::run(ctx, &command).await?,
        "fixed" => fixed::run(ctx, &command).await?,
        "get_discord_role" => get_discord_role::run(ctx, &command).await?,
        "gold_star" => gold_star::run(ctx, &command).await?,
        "good_morning" => good_morning::run(ctx, &command).await?,
        "good_night" => good_night::run(ctx, &command).await?,
        "image" => image::run(ctx, &command).await?,
        "infraction" => infraction::run(ctx, &command).await?,
        "levels" => levels::run(ctx, &command).await?,
        "link" => link::run(ctx, &command).await?,
        "logs" => logs::run(ctx, &command).await?,
        "member_count" => member_count::run(ctx, &command).await?,
        "open" => open::run(ctx, &command).await?,
        "patreon" => patreon::run(ctx, &command).await?,
        "rank" => rank::run(ctx, &command).await?,
        "reaction_role" => reaction_role::run(ctx, &command).await?,
        "ping" => ping::run(ctx, &command).await?,
        "reputation" => reputation::run(ctx, &command).await?,
        "rule" => rule::run(ctx, &command).await?,
        "saves" => saves::run(ctx, &command).await?,
        "scam" => scam::run(ctx, &command).await?,
        "server_info" => server_info::run(ctx, &command).await?,
        "spoilers" => spoilers::run(ctx, &command).await?,
        "stars" => stars::run(ctx, &command).await?,
        "support" => support::run(ctx, &command).await?,
        "xp" => xp::run(ctx, &command).await?,
        _ => unimplemented!("Command not implemented: {}", command_name),
    };

    Ok(())
}

pub async fn interaction_component(ctx: &Context, component: ComponentInteraction) -> Result<()> {
    match component.data.custom_id.as_str() {
        "support_ticket" => {}
        _ => unimplemented!("Component not implemented: {}", component.data.custom_id),
    }

    let version_input = CreateInputText::new(InputTextStyle::Short, "Game Version", "version")
        .required(true)
        .placeholder("1.0.0");

    let issue_input = CreateInputText::new(InputTextStyle::Paragraph, "Issue", "issue")
        .required(true)
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

    component
        .create_response(&ctx, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}

pub async fn interaction_modal(ctx: &Context, modal: ModalInteraction) -> Result<()> {
    println!("{} ran modal: {}", modal.user.tag(), modal.data.custom_id);

    match modal.data.custom_id.as_str() {
        "support_ticket" => {
            modals::support_ticket::run(ctx, &modal).await?;
        }
        _ => unimplemented!("Modal not implemented: {}", modal.data.custom_id),
    }

    Ok(())
}
