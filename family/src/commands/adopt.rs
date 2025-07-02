use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ResolvedValue, UserId,
};
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::relationships::Relationships;
use crate::{Error, Result};

use super::FamilyCommand;

pub struct Adopt;

#[async_trait]
impl FamilyCommand<UserId> for Adopt {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<UserId> {
        let target_user = match interaction.data.options()[0].value {
            ResolvedValue::User(user, _) => user,
            _ => unreachable!("User option must be a user"),
        };

        if interaction.user.id == target_user.id {
            return Err(Error::UserSelfAdopt);
        }

        if target_user.id == ctx.http.get_current_user().await?.id {
            return Err(Error::Zayden);
        }

        if target_user.bot {
            return Err(Error::Bot);
        }

        let row = match Manager::get_row(pool, interaction.user.id).await? {
            Some(row) => row,
            None => (&interaction.user).into(),
        };

        if !row.parent_ids.is_empty() {
            return Err(Error::AlreadyAdopted(target_user.id));
        }

        let relationship = row.relationship(interaction.user.id);
        if relationship != Relationships::None {
            return Err(Error::AlreadyRelated {
                target: target_user.id,
                relationship: Relationships::Parent,
            });
        }

        Ok(target_user.id)
    }

    fn register() -> CreateCommand {
        CreateCommand::new("adopt")
            .description("Adopt another user into your family.")
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "user", "The user to adopt.")
                    .required(true),
            )
    }
}
