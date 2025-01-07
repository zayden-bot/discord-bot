use serenity::all::{Context, ModalInteraction};
use sqlx::Postgres;
use ticket::SupportModal;

use crate::handler::Handler;
use crate::modals::{production_request, render_request, suggestions};
use crate::sqlx_lib::{GuildTable, PostgresPool};
use crate::Result;

impl Handler {
    pub async fn interaction_modal(ctx: &Context, modal: &ModalInteraction) -> Result<()> {
        println!("{} ran modal: {}", modal.user.name, modal.data.custom_id);

        let pool = PostgresPool::get(ctx).await;

        match modal.data.custom_id.as_str() {
            "production_request" => {
                production_request::run(ctx, modal).await?;
            }
            "render_request" => {
                render_request::run(ctx, modal).await?;
            }
            "suggestions_accept" => {
                suggestions::run(ctx, modal, true).await?;
            }
            "suggestions_reject" => {
                suggestions::run(ctx, modal, false).await?;
            }
            "support_ticket" => {
                SupportModal::run::<Postgres, GuildTable>(ctx, modal, &pool).await?;
            }
            _ => unimplemented!("Modal not implemented: {}", modal.data.custom_id),
        }

        Ok(())
    }
}
