use serenity::all::{CommandInteraction, ComponentInteraction, Context, ModalInteraction};

use crate::{
    components, global_commands::slash_commands::*, guild_commands::*, modals, Error, Result,
};

pub async fn interaction_command(ctx: &Context, command: &CommandInteraction) -> Result<()> {
    println!("{} ran command: {}", command.user.name, command.data.name);

    let result = match command.data.name.as_str() {
        "add_artist" => college_kings::add_artist::run(ctx, command).await,
        "availability_check" => college_kings::availability_check::run(ctx, command).await,
        "close" => college_kings::close::run(ctx, command).await,
        "cooldown" => college_kings::cooldown::run(ctx, command).await,
        "faq" => college_kings::faq::run(ctx, command).await,
        "fetch_suggestions" => college_kings::fetch_suggestions::run(ctx, command).await,
        "fixed" => college_kings::fixed::run(ctx, command).await,
        "get_discord_role" => college_kings::get_discord_role::run(ctx, command).await,
        "gold_star" => gold_star::run(ctx, command).await,
        "goodmorning" => college_kings::goodmorning::run(ctx, command).await,
        "goodnight" => college_kings::goodnight::run(ctx, command).await,
        "image" => college_kings::image::run(ctx, command).await,
        "infraction" => infraction::run(ctx, command).await,
        "levels" => levels::run(ctx, command).await,
        "link" => college_kings::link::run(ctx, command).await,
        "logs" => logs::run(ctx, command).await,
        "member_count" => member_count::run(ctx, command).await,
        "open" => college_kings::open::run(ctx, command).await,
        "patreon" => college_kings::patreon::run(ctx, command).await,
        "rank" => rank::run(ctx, command).await,
        "reaction_role" => reaction_role::run(ctx, command).await,
        "review" => college_kings_team::review::run(ctx, command).await,
        "ping" => ping::run(ctx, command).await,
        "reputation" => college_kings::reputation::run(ctx, command).await,
        "rule" => rule::run(ctx, command).await,
        "saves" => college_kings::saves::run(ctx, command).await,
        "scam" => scam::run(ctx, command).await,
        "server_info" => server_info::run(ctx, command).await,
        "spoilers" => college_kings::spoilers::run(ctx, command).await,
        "stars" => stars::run(ctx, command).await,
        "support" => college_kings::support::run(ctx, command).await,
        "xp" => xp::run(ctx, command).await,
        _ => Err(Error::CommandNotFound(command.data.name.clone()))?,
    };

    if let Err(e) = result {
        e.to_response(ctx, command).await?;
    }

    Ok(())
}

pub async fn interaction_component(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
    println!(
        "{} ran component: {}",
        component.user.name, component.data.custom_id
    );

    match component.data.custom_id.as_str() {
        "cron_available" => components::availability_check(ctx, component, true).await?,
        "cron_unavailable" => components::availability_check(ctx, component, false).await?,
        "faq" => components::faq(ctx, component, false).await?,
        "faq_ephemeral" => components::faq(ctx, component, true).await?,
        "levels_previous" => components::levels(ctx, component, "previous").await?,
        "levels_user" => components::levels(ctx, component, "user").await?,
        "levels_next" => components::levels(ctx, component, "next").await?,
        "production_request" => components::production_request(ctx, component).await?,
        "render_request" => components::render_request(ctx, component).await?,
        "suggestions_accept" | "accept" => components::suggestions(ctx, component, true).await?,
        "suggestions_reject" | "reject" => components::suggestions(ctx, component, false).await?,
        "suggestions_added" => components::suggestions(ctx, component, true).await?,
        "support_faq" => components::support_faq(ctx, component).await?,
        "support_close" => components::support_close(ctx, component).await?,
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
        "render_request" => {
            modals::render_request::run(ctx, modal).await?;
        }
        "suggestions_accept" => {
            modals::suggestions::run(ctx, modal, true).await?;
        }
        "suggestions_reject" => {
            modals::suggestions::run(ctx, modal, false).await?;
        }
        "support_ticket" => {
            modals::support_ticket::run(ctx, modal).await?;
        }
        _ => unimplemented!("Modal not implemented: {}", modal.data.custom_id),
    }

    Ok(())
}
