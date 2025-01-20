use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    EditInteractionResponse, Member, Mentionable, Permissions, Ready, ResolvedOption,
    ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{parse_options, SlashCommand};

use crate::guilds::ServersTable;
use crate::{Error, Result};

pub struct AddArtist;

#[async_trait]
impl SlashCommand<Error, Postgres> for AddArtist {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let mut options = parse_options(options);

        let Some(ResolvedValue::User(_, member)) = options.remove("user") else {
            unreachable!("User option is required");
        };

        let member = Member::from(member.ok_or(Error::MissingGuildId)?.to_owned());
        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let artist_role_id = ServersTable::get_row(pool, guild_id)
            .await
            .unwrap()
            .unwrap()
            .get_artist_role_id()
            .unwrap();

        member.add_role(&ctx, artist_role_id).await.unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content(format!("Added {} as an artist", member.mention())),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("add_artist")
            .description("Adds a user as an artist")
            .default_member_permissions(Permissions::MANAGE_MESSAGES)
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "user",
                    "The user to add as an artist",
                )
                .required(true),
            );

        Ok(command)
    }
}
