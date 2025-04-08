use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::PgPool;
use zayden_core::{ErrorResponse, SlashCommand, get_option_str};

use crate::Result;
use crate::handler::Handler;
use crate::modules::destiny2::endgame_analysis::slash_commands::{DimWishlist, TierList, Weapon};
use crate::modules::destiny2::info::Perk;
use crate::modules::destiny2::lfg::LfgCommand;
use crate::modules::events::live::Live;
use crate::modules::gambling::{
    Coinflip, Daily, Gift, HigherLower, Leaderboard, Profile, RPS, Roll, Send, Stats, TicTacToe,
    Work,
};
use crate::modules::levels::Levels;
use crate::modules::levels::slash_commands::{Rank, Xp};
use crate::modules::reaction_roles::ReactionRoleCommand;
use crate::modules::suggestions::FetchSuggestions;
use crate::modules::temp_voice::Voice;
use crate::modules::ticket::slash_commands::{SupportCommand, TicketCommand};

impl Handler {
    pub async fn interaction_command(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        let options = interaction.data.options();
        let options_str = get_option_str(&options);

        let result = match interaction.data.name.as_str() {
            // region Destiny 2
            "weapon" => Weapon::run(ctx, interaction, options, pool),
            "dimwishlist" => DimWishlist::run(ctx, interaction, options, pool),
            "lfg" => LfgCommand::run(ctx, interaction, options, pool),
            "tierlist" => TierList::run(ctx, interaction, options, pool),
            "perk" => Perk::run(ctx, interaction, options, pool),
            // endregion

            // region gambling
            "coinflip" => Coinflip::run(ctx, interaction, options, pool),
            "daily" => Daily::run(ctx, interaction, options, pool),
            "leaderboard" => Leaderboard::run(ctx, interaction, options, pool),
            "profile" => Profile::run(ctx, interaction, options, pool),
            "stats" => Stats::run(ctx, interaction, options, pool),
            "rps" => RPS::run(ctx, interaction, options, pool),
            "roll" => Roll::run(ctx, interaction, options, pool),
            "work" => Work::run(ctx, interaction, options, pool),
            "gift" => Gift::run(ctx, interaction, options, pool),
            "higherorlower" => HigherLower::run(ctx, interaction, options, pool),
            "send" => Send::run(ctx, interaction, options, pool),
            "tictactoe" => TicTacToe::run(ctx, interaction, options, pool),
            // endregion
            "fetch_suggestions" => FetchSuggestions::run(ctx, interaction, options, pool),
            "levels" => Levels::run(ctx, interaction, options, pool),
            "live" => Live::run(ctx, interaction, options, pool),
            "rank" => Rank::run(ctx, interaction, options, pool),
            "xp" => Xp::run(ctx, interaction, options, pool),

            // region: reaction_roles
            "reaction_role" => ReactionRoleCommand::run(ctx, interaction, options, pool),
            // endregion: reaction_roles
            "voice" => Voice::run(ctx, interaction, options, pool),

            // region: ticket
            "ticket" => TicketCommand::run(ctx, interaction, options, pool),
            "support" => SupportCommand::run(ctx, interaction, options, pool),
            // endregion: ticket
            _ => {
                println!("Unknown command: {}", interaction.data.name);
                return Ok(());
            }
        }
        .await;

        if let Err(e) = result {
            println!(
                "{} ran command: {}{}",
                interaction.user.name, interaction.data.name, options_str
            );

            let msg = e.to_response();
            let _ = interaction.defer_ephemeral(ctx).await;

            interaction
                .edit_response(ctx, EditInteractionResponse::new().content(msg))
                .await
                .unwrap();
        }

        Ok(())
    }
}
