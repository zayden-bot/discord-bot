use serenity::all::{CommandInteraction, ComponentInteraction, Context, ModalInteraction};

use crate::{
    components, global_commands::slash_commands::*, guild_commands::slash_commands::*, modals,
    Error, Result,
};

pub async fn interaction_command(ctx: &Context, command: &CommandInteraction) -> Result<()> {
    println!("{} ran command: {}", command.user.name, command.data.name);

    match command.data.name.as_str() {
        "add_artist" => add_artist::run(ctx, command).await?,
        "close" => close::run(ctx, command).await?,
        "faq" => faq::run(ctx, command).await?,
        "fetch_suggestions" => fetch_suggestions::run(ctx, command).await?,
        "fixed" => fixed::run(ctx, command).await?,
        "get_discord_role" => get_discord_role::run(ctx, command).await?,
        "gold_star" => gold_star::run(ctx, command).await?,
        "good_morning" => good_morning::run(ctx, command).await?,
        "good_night" => good_night::run(ctx, command).await?,
        "image" => image::run(ctx, command).await?,
        "infraction" => infraction::run(ctx, command).await?,
        "levels" => levels::run(ctx, command).await?,
        "link" => link::run(ctx, command).await?,
        "logs" => logs::run(ctx, command).await?,
        "member_count" => member_count::run(ctx, command).await?,
        "open" => open::run(ctx, command).await?,
        "patreon" => patreon::run(ctx, command).await?,
        "rank" => rank::run(ctx, command).await?,
        "reaction_role" => reaction_role::run(ctx, command).await?,
        "ping" => ping::run(ctx, command).await?,
        "reputation" => reputation::run(ctx, command).await?,
        "rule" => rule::run(ctx, command).await?,
        "saves" => saves::run(ctx, command).await?,
        "scam" => scam::run(ctx, command).await?,
        "server_info" => server_info::run(ctx, command).await?,
        "spoilers" => spoilers::run(ctx, command).await?,
        "stars" => stars::run(ctx, command).await?,
        "support" => support::run(ctx, command).await?,
        "test" => test::run(ctx, command).await?,
        "xp" => xp::run(ctx, command).await?,
        _ => Err(Error::CommandNotFound(command.data.name.clone()))?,
    };

    Ok(())
}

pub async fn interaction_component(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
    println!(
        "{} ran component: {}",
        component.user.name, component.data.custom_id
    );

    match component.data.custom_id.as_str() {
        "cron_available" | "cron_unavailable" => {
            components::availability_check(ctx, component).await?
        }
        "faq" | "faq_ephemeral" => components::faq(ctx, component).await?,
        "production_request" => components::production_request(ctx, component).await?,
        "support_ticket" => components::support_ticket(ctx, component).await?,
        _ => unimplemented!("Component not implemented: {}", component.data.custom_id),
    }

    Ok(())
}

pub async fn interaction_modal(ctx: &Context, modal: &ModalInteraction) -> Result<()> {
    println!("{} ran modal: {}", modal.user.name, modal.data.custom_id);

    match modal.data.custom_id.as_str() {
        "production_request" => {
            modals::production_request::run(ctx, modal).await?;
        }
        "support_ticket" => {
            modals::support_ticket::run(ctx, modal).await?;
        }
        _ => unimplemented!("Modal not implemented: {}", modal.data.custom_id),
    }

    Ok(())
}
