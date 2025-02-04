mod command;
mod component;
mod modal;

use serenity::all::{Context, Interaction};
use sqlx::PgPool;

use crate::Result;

use super::Handler;

impl Handler {
    pub async fn interaction_create(
        ctx: &Context,
        interaction: Interaction,
        pool: &PgPool,
    ) -> Result<()> {
        match &interaction {
            Interaction::Command(command) => Self::interaction_command(ctx, command).await?,
            Interaction::Component(component) => {
                Self::interaction_component(ctx, component, pool).await?
            }
            Interaction::Modal(modal) => Self::interaction_modal(ctx, modal, pool).await?,
            _ => unimplemented!("Interaction not implemented: {:?}", interaction.kind()),
        };

        Ok(())
    }
}
