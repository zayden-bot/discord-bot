use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, EditInteractionResponse, Permissions,
    ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::ReactionRolesTable;

pub struct ReactionRoleCommand;

#[async_trait]
impl SlashCommand<Error, Postgres> for ReactionRoleCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        reaction_roles::ReactionRoleCommand::run::<Postgres, ReactionRolesTable>(
            ctx,
            interaction,
            pool,
        )
        .await?;

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content("Success."))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let command = reaction_roles::ReactionRoleCommand::register()
            .default_member_permissions(Permissions::MANAGE_MESSAGES);

        Ok(command)
    }
}
