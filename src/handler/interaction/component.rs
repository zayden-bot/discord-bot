use serenity::all::{
    ComponentInteraction, Context, CreateInteractionResponseFollowup, Mentionable,
};
use sqlx::PgPool;
use suggestions::Suggestions;
use zayden_core::{Component, ErrorResponse};

use crate::handler::Handler;
use crate::modules::levels::Levels;
use crate::modules::ticket::Ticket;
use crate::{components, Result, SUPER_USERS};

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
            "cron_available" => components::availability_check(ctx, interaction, true).await,
            "cron_unavailable" => components::availability_check(ctx, interaction, false).await,
            "faq" => components::faq(ctx, interaction, false).await,
            "faq_ephemeral" => components::faq(ctx, interaction, true).await,
            "levels_previous" | "levels_user" | "levels_next" => {
                Levels::run(ctx, interaction, pool).await
            }
            "production_request" => components::production_request(ctx, interaction).await,
            "render_request" => components::render_request(ctx, interaction, pool).await,
            "suggestions_accept" | "suggestions_added" | "accept" => {
                Suggestions::components(ctx, interaction, true).await;
                Ok(())
            }
            "suggestions_reject" | "reject" => {
                Suggestions::components(ctx, interaction, false).await;
                Ok(())
            }

            //region Family
            // "adopt_accept" => AdoptComponent::accept(ctx, component).await,
            // "adopt_decline" => AdoptComponent::decline(ctx, component).await,

            // "marry_accept" => MarryComponent::accept(ctx, component).await,
            // "marry_decline" => MarryComponent::decline(ctx, component).await,
            //endregion

            //region: Misc
            "sleep_confirm" => Ok(()),
            "sleep_cancel" => Ok(()),
            //endregion: Misc

            //region: Ticket
            "ticket_create" | "support_ticket" => Ticket::ticket_create(ctx, interaction).await,
            "support_close" => Ticket::support_close(ctx, interaction).await,
            "support_faq" => Ticket::support_faq(ctx, interaction, pool).await,
            //endregion: Ticket
            _ => unimplemented!("Component not implemented: {}", interaction.data.custom_id),
        };

        if let Err(e) = result {
            let msg = e.to_response();
            if msg.is_empty() {
                interaction
                    .create_followup(
                        ctx,
                        CreateInteractionResponseFollowup::new().content(format!(
                            "An error occurred. Please contact {} if this issue persists.",
                            SUPER_USERS[0].mention()
                        )),
                    )
                    .await
                    .unwrap();
                return Err(e);
            }
            interaction
                .create_followup(
                    ctx,
                    CreateInteractionResponseFollowup::new()
                        .content(msg)
                        .ephemeral(true),
                )
                .await
                .unwrap();
        }

        Ok(())
    }
}
