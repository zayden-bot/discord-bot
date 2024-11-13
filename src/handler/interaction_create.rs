use serenity::all::{
    CommandInteraction, Context, EditInteractionResponse, Mentionable, ModalInteraction,
};
use zayden_core::{ErrorResponse, SlashCommand};

use crate::modules::family::slash_commands::{
    AdoptCommand, BlockCommand, ChildrenCommand, MarryCommand, ParentsCommand, PartnersCommand,
    RelationshipCommand, SiblingsCommand, TreeCommand, UnblockCommand,
};
use crate::modules::gold_star::slash_commands::{GiveStarCommand, StarsCommand};
use crate::modules::{misc, patreon};
use crate::modules::{moderation, reaction_roles};
use crate::{
    global_commands::slash_commands::*, guild_commands::*, modals, Error, Result, SUPER_USERS,
};

pub async fn interaction_command(ctx: &Context, command: &CommandInteraction) -> Result<()> {
    println!("{} ran command: {}", command.user.name, command.data.name);

    let result = match command.data.name.as_str() {
        "add_artist" => college_kings::add_artist::run(ctx, command).await,
        "availability_check" => college_kings::availability_check::run(ctx, command).await,
        "close" => college_kings::close::run(ctx, command).await,
        "faq" => college_kings::faq::run(ctx, command).await,
        "fetch_suggestions" => college_kings::fetch_suggestions::run(ctx, command).await,
        "fixed" => college_kings::fixed::run(ctx, command).await,
        "get_discord_role" => college_kings::get_discord_role::run(ctx, command).await,
        "goodmorning" => college_kings::goodmorning::run(ctx, command).await,
        "goodnight" => college_kings::goodnight::run(ctx, command).await,
        "image" => college_kings::image::run(ctx, command).await,
        "levels" => levels::run(ctx, command).await,
        "member_count" => member_count::run(ctx, command).await,
        "open" => college_kings::open::run(ctx, command).await,
        "rank" => rank::run(ctx, command).await,
        "review" => college_kings_team::review::run(ctx, command).await,
        "ping" => ping::run(ctx, command).await,
        "reputation" => college_kings::reputation::run(ctx, command).await,
        "rule" => rule::run(ctx, command).await,
        "saves" => college_kings::saves::run(ctx, command).await,
        "scam" => scam::run(ctx, command).await,
        "server_info" => server_info::run(ctx, command).await,
        "spoilers" => college_kings::spoilers::run(ctx, command).await,
        "support" => college_kings::support::run(ctx, command).await,
        "xp" => xp::run(ctx, command).await,

        //region Family
        "adopt" => AdoptCommand::run(ctx, command).await,
        "block" => BlockCommand::run(ctx, command).await,
        "children" => ChildrenCommand::run(ctx, command).await,
        "marry" => MarryCommand::run(ctx, command).await,
        "parents" => ParentsCommand::run(ctx, command).await,
        "partners" => PartnersCommand::run(ctx, command).await,
        "relationship" => RelationshipCommand::run(ctx, command).await,
        "siblings" => SiblingsCommand::run(ctx, command).await,
        "tree" => TreeCommand::run(ctx, command).await,
        "unblock" => UnblockCommand::run(ctx, command).await,
        // endregion

        // region Gold Stars
        "give_star" => GiveStarCommand::run(ctx, command).await,
        "stars" => StarsCommand::run(ctx, command).await,
        // endregion

        //region: misc
        "sleep" => misc::Sleep::run(ctx, command).await,
        "link" => misc::Link::run(ctx, command).await,
        //endregion: misc

        //region: moderation
        "infraction" => moderation::Infraction::run(ctx, command).await,
        "logs" => moderation::Logs::run(ctx, command).await,
        "rules" => moderation::RulesCommand::run(ctx, command).await,
        //endregion: moderation

        //region: patreon
        "patreon" => patreon::Patreon::run(ctx, command).await,
        //endregion: patreon

        //region: reaction_roles
        "reaction_role" => reaction_roles::ReactionRoleCommand::run(ctx, command).await,
        //endregion: reaction_roles
        _ => Err(Error::UnknownCommand(command.data.name.clone())),
    };

    if let Err(e) = result {
        let msg = e.to_response();
        let _ = command.defer(ctx).await;
        if msg.is_empty() {
            command
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content(format!(
                        "An error occurred. Please contact {} if this issue persists.",
                        SUPER_USERS[0].mention()
                    )),
                )
                .await?;
            return Err(e);
        }
        command
            .edit_response(ctx, EditInteractionResponse::new().content(msg))
            .await?;
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
