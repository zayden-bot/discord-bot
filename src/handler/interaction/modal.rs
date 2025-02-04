use serenity::all::{Context, ModalInteraction};
use sqlx::{PgPool, Postgres};
use suggestions::Suggestions;
use ticket::TicketModal;

use crate::handler::Handler;
use crate::modals::{production_request, render_request};
use crate::modules::ticket::TicketTable;
use crate::sqlx_lib::GuildTable;
use crate::{Error, Result};

impl Handler {
    pub async fn interaction_modal(
        ctx: &Context,
        modal: &ModalInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        println!("{} ran modal: {}", modal.user.name, modal.data.custom_id);

        match modal.data.custom_id.as_str() {
            "production_request" => {
                production_request::run(ctx, modal).await?;
            }
            "render_request" => {
                render_request::run(ctx, modal).await?;
            }
            "suggestions_accept" => {
                Suggestions::modal(ctx, modal, true).await;
            }
            "suggestions_reject" => {
                Suggestions::modal(ctx, modal, false).await;
            }
            "create_ticket" => {
                TicketModal::run::<Postgres, GuildTable, TicketTable>(ctx, modal, pool)
                    .await
                    .map_err(Error::from)?;
            }
            _ => unimplemented!("Modal not implemented: {}", modal.data.custom_id),
        }

        Ok(())
    }
}
