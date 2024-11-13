use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, EditInteractionResponse, Permissions, Ready,
};
use sqlx::Postgres;
use zayden_core::SlashCommand;

use crate::sqlx_lib::PostgresPool;
use crate::{Error, Result};

use super::ReactionRolesTable;

pub struct ReactionRoleCommand;

#[async_trait]
impl SlashCommand<Error> for ReactionRoleCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        reaction_roles::ReactionRoleCommand::run::<Postgres, ReactionRolesTable>(
            ctx,
            interaction,
            &pool,
        )
        .await?;

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content("Success."))
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = reaction_roles::ReactionRoleCommand::register()
            .default_member_permissions(Permissions::MANAGE_MESSAGES);

        Ok(command)
    }
}
