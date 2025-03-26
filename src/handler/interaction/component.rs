use serenity::all::{ComponentInteraction, Context, EditInteractionResponse};
use sqlx::PgPool;
use suggestions::Suggestions;
use zayden_core::ErrorResponse;

use crate::Result;
use crate::handler::Handler;
use crate::modules::ticket::Ticket;

impl Handler {
    pub async fn interaction_component(
        ctx: &Context,
        interaction: &ComponentInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        println!(
            "{} ran component: {}",
            interaction.user.name, interaction.data.custom_id
        );

        let result = match interaction.data.custom_id.as_str() {
            "suggestions_accept" | "suggestions_added" | "accept" => {
                Suggestions::components(ctx, interaction, true).await;
                Ok(())
            }
            "suggestions_reject" | "reject" => {
                Suggestions::components(ctx, interaction, false).await;
                Ok(())
            }

            //region: Ticket
            "ticket_create" | "support_ticket" => Ticket::ticket_create(ctx, interaction).await,
            "support_close" => Ticket::support_close(ctx, interaction).await,
            "support_faq" => Ticket::support_faq(ctx, interaction, pool).await,
            //endregion: Ticket
            _ => unimplemented!("Component not implemented: {}", interaction.data.custom_id),
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
