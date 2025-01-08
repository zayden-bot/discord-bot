use serenity::all::{
    ComponentInteraction, Context, CreateInteractionResponseFollowup, Mentionable,
};
use sqlx::PgPool;
use zayden_core::ErrorResponse;

use crate::handler::Handler;
use crate::modules::ticket::components::{support_close, support_faq, support_ticket};
// use crate::modules::family::components::{AdoptComponent, MarryComponent};
use crate::{components, Result, SUPER_USERS};

impl Handler {
    pub async fn interaction_component(
        ctx: &Context,
        component: &ComponentInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        println!(
            "{} ran component: {}",
            component.user.name, component.data.custom_id
        );

        let result = match component.data.custom_id.as_str() {
            "cron_available" => components::availability_check(ctx, component, true).await,
            "cron_unavailable" => components::availability_check(ctx, component, false).await,
            "faq" => components::faq(ctx, component, false).await,
            "faq_ephemeral" => components::faq(ctx, component, true).await,
            "levels_previous" => components::levels(ctx, component, "previous").await,
            "levels_user" => components::levels(ctx, component, "user").await,
            "levels_next" => components::levels(ctx, component, "next").await,
            "production_request" => components::production_request(ctx, component).await,
            "render_request" => components::render_request(ctx, component).await,
            "suggestions_accept" | "accept" => components::suggestions(ctx, component, true).await,
            "suggestions_reject" | "reject" => components::suggestions(ctx, component, false).await,
            "suggestions_added" => components::suggestions(ctx, component, true).await,

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
            "support_close" => support_close(ctx, component).await,
            "support_faq" => support_faq(ctx, component, pool).await,
            "support_ticket" => support_ticket(ctx, component).await,
            //endregion: Ticket
            _ => unimplemented!("Component not implemented: {}", component.data.custom_id),
        };

        if let Err(e) = result {
            let msg = e.to_response();
            if msg.is_empty() {
                component
                    .create_followup(
                        ctx,
                        CreateInteractionResponseFollowup::new().content(format!(
                            "An error occurred. Please contact {} if this issue persists.",
                            SUPER_USERS[0].mention()
                        )),
                    )
                    .await?;
                return Err(e);
            }
            component
                .create_followup(
                    ctx,
                    CreateInteractionResponseFollowup::new()
                        .content(msg)
                        .ephemeral(true),
                )
                .await?;
        }

        Ok(())
    }
}
