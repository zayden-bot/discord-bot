use async_trait::async_trait;
use serenity::all::Permissions;
use serenity::all::{CommandInteraction, Context, CreateCommand};
use sqlx::{Database, Pool};

use crate::FamilyManager;
use crate::Result;

use super::FamilyCommand;

pub struct ResetFamily;

#[async_trait]
impl FamilyCommand<()> for ResetFamily {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        _ctx: &Context,
        _interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        Manager::reset(pool).await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("resetfamily")
            .description("Resets the family tree(s) in guild")
            .default_member_permissions(Permissions::ADMINISTRATOR)
    }
}
