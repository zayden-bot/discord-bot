use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ResolvedValue, UserId,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::family_manager::FamilyManager;
use crate::relationships::Relationships;
use crate::{Error, Result};

use super::FamilyCommand;

pub struct RelationshipResponse {
    pub other_id: UserId,
    pub user_id: UserId,
    pub relationship: Relationships,
}

pub struct Relationship;

#[async_trait]
impl FamilyCommand<RelationshipResponse> for Relationship {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<RelationshipResponse> {
        interaction.defer(ctx).await?;

        let options = interaction.data.options();
        let options = parse_options(options);

        let user = match options.get("user") {
            Some(ResolvedValue::User(user, _)) => *user,
            _ => unreachable!("User option is required and must be a user."),
        };

        let other = match options.get("other") {
            Some(ResolvedValue::User(user, _)) => *user,
            _ => &interaction.user,
        };

        if user == other {
            return Err(Error::SameUser(user.id));
        }

        let user_info = match Manager::get_row(pool, user.id).await? {
            Some(row) => row,
            None => user.into(),
        };

        let relationship = user_info.relationship(other.id);

        Ok(RelationshipResponse {
            other_id: other.id,
            user_id: user.id,
            relationship,
        })
    }

    fn register() -> CreateCommand {
        CreateCommand::new("relationship")
            .description("View the relationship between two users.")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "user",
                    "The user you want to view the relationship of.",
                )
                .required(true),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "other",
                "The other user you want to view the relationship of.",
            ))
    }
}
