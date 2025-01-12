use serenity::all::{CommandInteraction, Context, EditInteractionResponse, Mentionable};
use zayden_core::{get_option_str, ErrorResponse, SlashCommand};

use crate::handler::Handler;
// use crate::modules::family::slash_commands::{
//     AdoptCommand, BlockCommand, ChildrenCommand, MarryCommand, ParentsCommand, PartnersCommand,
//     RelationshipCommand, SiblingsCommand, TreeCommand, UnblockCommand,
// };
use crate::global_commands::slash_commands::{
    levels, member_count, ping, rank, scam, server_info, xp,
};
use crate::guild_commands::{college_kings, college_kings_team};
use crate::modules::gold_star::slash_commands::{GiveStarCommand, StarsCommand};
use crate::modules::{misc, patreon};
use crate::modules::{moderation, reaction_roles};
use crate::{Error, Result, SUPER_USERS};

impl Handler {
    pub async fn interaction_command(ctx: &Context, command: &CommandInteraction) -> Result<()> {
        let options = command.data.options();
        let options_str = get_option_str(&options);

        println!(
            "{} ran command: {}{}",
            command.user.name, command.data.name, options_str
        );

        let result = match command.data.name.as_str() {
            "add_artist" => college_kings::add_artist::run(ctx, command).await,
            "availability_check" => college_kings::availability_check::run(ctx, command).await,
            "faq" => college_kings::faq::run(ctx, command).await,
            "fetch_suggestions" => college_kings::fetch_suggestions::run(ctx, command).await,
            "get_discord_role" => college_kings::get_discord_role::run(ctx, command).await,
            "goodmorning" => college_kings::goodmorning::run(ctx, command).await,
            "goodnight" => college_kings::goodnight::run(ctx, command).await,
            "image" => college_kings::image::run(ctx, command).await,
            "levels" => levels::run(ctx, command).await,
            "member_count" => member_count::run(ctx, command).await,
            "rank" => rank::run(ctx, command).await,
            "review" => college_kings_team::review::run(ctx, command).await,
            "ping" => ping::run(ctx, command).await,
            "reputation" => college_kings::reputation::run(ctx, command).await,
            "saves" => college_kings::saves::run(ctx, command).await,
            "scam" => scam::run(ctx, command).await,
            "server_info" => server_info::run(ctx, command).await,
            "spoilers" => college_kings::spoilers::run(ctx, command).await,
            "xp" => xp::run(ctx, command).await,

            //region Family
            // "adopt" => AdoptCommand::run(ctx, command, options).await,
            // "block" => BlockCommand::run(ctx, command, options).await,
            // "children" => ChildrenCommand::run(ctx, command, options).await,
            // "marry" => MarryCommand::run(ctx, command, options).await,
            // "parents" => ParentsCommand::run(ctx, command, options).await,
            // "partners" => PartnersCommand::run(ctx, command, options).await,
            // "relationship" => RelationshipCommand::run(ctx, command, options).await,
            // "siblings" => SiblingsCommand::run(ctx, command, options).await,
            // "tree" => TreeCommand::run(ctx, command, options).await,
            // "unblock" => UnblockCommand::run(ctx, command, options).await,
            // endregion

            // region Gold Stars
            "give_star" => GiveStarCommand::run(ctx, command, options).await,
            "stars" => StarsCommand::run(ctx, command, options).await,
            // endregion

            //region: misc
            "sleep" => misc::Sleep::run(ctx, command, options).await,
            "link" => misc::Link::run(ctx, command, options).await,
            //endregion: misc

            //region: moderation
            "infraction" => moderation::Infraction::run(ctx, command, options).await,
            "logs" => moderation::Logs::run(ctx, command, options).await,
            "rules" => moderation::RulesCommand::run(ctx, command, options).await,
            //endregion: moderation

            //region: patreon
            "patreon" => patreon::Patreon::run(ctx, command, options).await,
            //endregion: patreon

            //region: reaction_roles
            "reaction_role" => {
                reaction_roles::ReactionRoleCommand::run(ctx, command, options).await
            }
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
                    .await
                    .unwrap();
                return Err(e);
            }
            command
                .edit_response(ctx, EditInteractionResponse::new().content(msg))
                .await
                .unwrap();
        }

        Ok(())
    }
}
