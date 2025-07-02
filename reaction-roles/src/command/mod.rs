use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ReactionType, ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

mod add;
mod remove;

pub use crate::error::{Error, Result};
pub use crate::reaction_roles_manager::ReactionRolesManager;

pub struct ReactionRoleCommand;

impl ReactionRoleCommand {
    pub async fn run<Db: Database, Manager: ReactionRolesManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let command = interaction.data.options().remove(0);

        let options = match command.value {
            ResolvedValue::SubCommand(options) => options,
            _ => unreachable!("Subcommand is required"),
        };
        let mut options = parse_options(options);

        let channel_id = match options.remove("channel") {
            Some(ResolvedValue::Channel(channel)) => channel.id,
            _ => interaction.channel_id,
        };

        let Some(ResolvedValue::String(emoji)) = options.remove("emoji") else {
            unreachable!("Emoji is required");
        };

        let reaction = ReactionType::try_from(emoji)?;

        match command.name {
            "add" => {
                Self::add::<Db, Manager>(ctx, pool, guild_id, channel_id, reaction, options).await?
            }
            "remove" => {
                Self::remove::<Db, Manager>(ctx, pool, channel_id, guild_id, reaction, options)
                    .await?;
            }
            _ => unreachable!("Invalid subcommand name"),
        };

        Ok(())
    }

    pub fn register() -> CreateCommand {
        let add =
            CreateCommandOption::new(CommandOptionType::SubCommand, "add", "Adds a reaction role")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "emoji",
                        "The emoji of the reaction role",
                    )
                    .required(true),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Role,
                        "role",
                        "The role to add when the emoji is reacted to",
                    )
                    .required(true),
                )
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::Channel,
                    "channel",
                    "The channel the message is in",
                ))
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::String,
                    "message_id",
                    "The message id of the reaction role message",
                ));

        let remove = CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "remove",
            "Removes a reaction role",
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "message_id",
                "The message id of the reaction role message",
            )
            .required(true),
        )
        .add_sub_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "emoji",
                "The emoji of the reaction role",
            )
            .required(true),
        )
        .add_sub_option(CreateCommandOption::new(
            CommandOptionType::Channel,
            "channel",
            "The channel the message is in",
        ));

        CreateCommand::new("reaction_role")
            .description("Adds or removes a reaction role")
            .add_option(add)
            .add_option(remove)
    }
}
