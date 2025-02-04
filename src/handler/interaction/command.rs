use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use zayden_core::{get_option_str, ErrorResponse, SlashCommand};

use crate::global_commands::slash_commands::{MemberCount, Ping, Scam, ServerInfo};
use crate::guild_commands::college_kings::{
    AddArtist, AvailabilityCheck, Faq, GetDiscordRole, Greetings, Reputation, Saves, Spoilers,
};
use crate::guild_commands::college_kings_team::Review;
use crate::handler::Handler;
use crate::modules::gold_star::slash_commands::{GiveStarCommand, StarsCommand};
use crate::modules::levels::slash_commands::{Rank, Xp};
use crate::modules::levels::Levels;
use crate::modules::misc::{Link, Sleep};
use crate::modules::moderation::{Infraction, Logs, RulesCommand};
use crate::modules::patreon::Patreon;
use crate::modules::reaction_roles::ReactionRoleCommand;
use crate::modules::suggestions::FetchSuggestions;
use crate::modules::ticket::slash_commands::{SupportCommand, TicketCommand};
use crate::sqlx_lib::PostgresPool;
use crate::Result;

impl Handler {
    pub async fn interaction_command(ctx: &Context, command: &CommandInteraction) -> Result<()> {
        let options = command.data.options();
        let options_str = get_option_str(&options);

        println!(
            "{} ran command: {}{}",
            command.user.name, command.data.name, options_str
        );

        let pool = PostgresPool::get(ctx).await;

        let result = match command.data.name.as_str() {
            "add_artist" => AddArtist::run(ctx, command, options, &pool),
            "availability_check" => AvailabilityCheck::run(ctx, command, options, &pool),
            "faq" => Faq::run(ctx, command, options, &pool),
            "fetch_suggestions" => FetchSuggestions::run(ctx, command, options, &pool),
            "get_discord_role" => GetDiscordRole::run(ctx, command, options, &pool),
            "good" => Greetings::run(ctx, command, options, &pool),
            "levels" => Levels::run(ctx, command, options, &pool),
            "member_count" => MemberCount::run(ctx, command, options, &pool),
            "rank" => Rank::run(ctx, command, options, &pool),
            "review" => Review::run(ctx, command, options, &pool),
            "ping" => Ping::run(ctx, command, options, &pool),
            "reputation" => Reputation::run(ctx, command, options, &pool),
            "saves" => Saves::run(ctx, command, options, &pool),
            "scam" => Scam::run(ctx, command, options, &pool),
            "server_info" => ServerInfo::run(ctx, command, options, &pool),
            "spoilers" => Spoilers::run(ctx, command, options, &pool),
            "xp" => Xp::run(ctx, command, options, &pool),

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
            "give_star" => GiveStarCommand::run(ctx, command, options, &pool),
            "stars" => StarsCommand::run(ctx, command, options, &pool),
            // endregion

            //region: misc
            "sleep" => Sleep::run(ctx, command, options, &pool),
            "link" => Link::run(ctx, command, options, &pool),
            //endregion: misc

            //region: moderation
            "infraction" => Infraction::run(ctx, command, options, &pool),
            "logs" => Logs::run(ctx, command, options, &pool),
            "rules" => RulesCommand::run(ctx, command, options, &pool),
            //endregion: moderation

            //region: patreon
            "patreon" => Patreon::run(ctx, command, options, &pool),
            //endregion: patreon

            //region: reaction_roles
            "reaction_role" => ReactionRoleCommand::run(ctx, command, options, &pool),
            //endregion: reaction_roles

            //region: ticket
            "ticket" => TicketCommand::run(ctx, command, options, &pool),
            "support" => SupportCommand::run(ctx, command, options, &pool),
            //endregion: ticket
            _ => {
                println!("Unknown command: {}", command.data.name);
                return Ok(());
            }
        }
        .await;

        if let Err(e) = result {
            let msg = e.to_response();
            let _ = command.defer_ephemeral(ctx).await;

            command
                .edit_response(ctx, EditInteractionResponse::new().content(msg))
                .await
                .unwrap();
        }

        Ok(())
    }
}
