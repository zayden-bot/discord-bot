use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::PgPool;
use zayden_core::{Autocomplete, ErrorResponse};

use crate::Result;
use crate::handler::Handler;
use crate::modules::destiny2::endgame_analysis::slash_commands::{TierList, Weapon};
use crate::modules::destiny2::lfg::LfgCommand;

impl Handler {
    pub async fn interaction_autocomplete(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        let option = interaction.data.autocomplete().unwrap();

        let result = match interaction.data.name.as_str() {
            "lfg" => LfgCommand::autocomplete(ctx, interaction, option, pool).await,
            "weapon" => Weapon::autocomplete(ctx, interaction, option, pool).await,
            "tierlist" => TierList::autocomplete(ctx, interaction, option, pool).await,
            _ => {
                println!("Unknown command: {}", interaction.data.name);
                return Ok(());
            }
        };

        if let Err(e) = result {
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
