use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::{Error, Result};

use super::FamilyCommand;

pub struct Block;

#[async_trait]
impl FamilyCommand<()> for Block {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        _ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let user = match interaction.data.options()[0].value {
            ResolvedValue::User(user, _) => user,
            _ => unreachable!("User option was not a user."),
        };

        if &interaction.user == user {
            return Err(Error::UserSelfBlock);
        }

        let mut row = match Manager::get_row(pool, interaction.user.id).await? {
            Some(row) => row,
            None => (&interaction.user).into(),
        };

        row.add_blocked(user.id);
        row.save::<Db, Manager>(pool).await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("block")
            .description("Blocks a user from being able to adopt/marry/etc you.")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to block.",
            ))
    }
}

pub struct Unblock;

#[async_trait]
impl FamilyCommand<()> for Unblock {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        _ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let user = match interaction.data.options()[0].value {
            ResolvedValue::User(user, _) => user,
            _ => unreachable!("User option was not a user."),
        };

        if &interaction.user == user {
            return Err(Error::UserSelfBlock);
        }

        let mut row = match Manager::get_row(pool, interaction.user.id).await? {
            Some(row) => row,
            None => (&interaction.user).into(),
        };

        row.remove_blocked(user.id);
        row.save::<Db, Manager>(pool).await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("unblock")
            .description("Unblocks a user from being able to adopt/marry/etc you.")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to unblock.",
            ))
    }
}
