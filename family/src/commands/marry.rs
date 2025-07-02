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

const MAX_PARTNERS: usize = 1;

pub struct Marry;

#[async_trait]
impl FamilyCommand<UserId> for Marry {
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
            return Err(Error::UserSelfMarry);
        }

        if target_user.id == ctx.http.get_current_user().await?.id {
            return Err(Error::Zayden);
        }

        if target_user.bot {
            return Err(Error::Bot);
        }

        if let Some(row) = Manager::get_row(pool, interaction.user.id).await? {
            let relationship = row.relationship(target_user.id);

            if relationship != Relationships::None {
                return Err(Error::AlreadyRelated {
                    target: target_user.id,
                    relationship,
                });
            }

            if row.partner_ids.len() >= MAX_PARTNERS {
                return Err(Error::MaxPartners);
            }
        }

        if let Some(row) = Manager::get_row(pool, target_user.id).await? {
            if row.partner_ids.len() >= MAX_PARTNERS {
                return Err(Error::MaxPartners);
            }
        }

        Ok(target_user.id)
    }

    fn register() -> CreateCommand {
        CreateCommand::new("marry")
            .description("Propose to another Discord user")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "user",
                    "The user you want to propose to",
                )
                .required(true),
            )
    }
}
