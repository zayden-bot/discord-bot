use serenity::all::{Context, EditInteractionResponse, ModalInteraction};
use sqlx::{PgPool, Postgres};
use suggestions::Suggestions;
use ticket::TicketModal;
use zayden_core::ErrorResponse;

use crate::handler::Handler;
use crate::modules::ticket::TicketTable;
use crate::sqlx_lib::GuildTable;
use crate::{Error, Result};

impl Handler {
    pub async fn interaction_modal(
        ctx: &Context,
        interaction: &ModalInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        println!(
            "{} ran modal: {}",
            interaction.user.name, interaction.data.custom_id
        );

        let result = match interaction.data.custom_id.as_str() {
            "suggestions_accept" => {
                Suggestions::modal(ctx, interaction, true).await;
                Ok(())
            }
            "suggestions_reject" => {
                Suggestions::modal(ctx, interaction, false).await;
                Ok(())
            }
            "create_ticket" => {
                TicketModal::run::<Postgres, GuildTable, TicketTable>(ctx, interaction, pool)
                    .await
                    .map_err(Error::from)
            }
            _ => unimplemented!("Modal not implemented: {}", interaction.data.custom_id),
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
