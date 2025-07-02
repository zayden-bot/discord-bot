use async_trait::async_trait;
use futures::{stream, StreamExt, TryStreamExt};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Mentionable, ResolvedOption, ResolvedValue, UserId,
};
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::{Error, Result};

use super::FamilyCommand;

pub struct Siblings;

#[async_trait]
impl FamilyCommand<(UserId, Vec<String>)> for Siblings {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<(UserId, Vec<String>)> {
        let user = match interaction.data.options().first() {
            Some(ResolvedOption {
                value: ResolvedValue::User(user, _),
                ..
            }) => *user,
            _ => &interaction.user,
        };

        let row = match Manager::get_row(pool, user.id).await? {
            Some(row) => row,
            None => (&interaction.user).into(),
        };

        if row.parent_ids.is_empty() {
            if user == &interaction.user {
                return Err(Error::SelfNoParents);
            }

            return Err(Error::NoParents(user.id));
        }

        let siblings: Vec<String> = stream::iter(row.parent_ids)
            .then(|id| async move {
                if let Some(row) = Manager::get_row(pool, id).await? {
                    for sib_id in row.children_ids {
                        if sib_id != row.id {
                            let user_id = UserId::new(sib_id as u64);
                            let user = user_id.to_user(ctx).await?;

                            return Ok::<String, Error>(user.mention().to_string());
                        }
                    }
                }

                Err(Error::NoData(UserId::new(id as u64)))
            })
            .try_collect()
            .await?;

        Ok((user.id, siblings))
    }

    fn register() -> CreateCommand {
        CreateCommand::new("siblings")
            .description("List who your siblings are.")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to check. Leave blank to check yourself.",
            ))
    }
}
