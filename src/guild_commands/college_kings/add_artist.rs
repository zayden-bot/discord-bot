use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Permissions, Ready, ResolvedValue,
};
use zayden_core::parse_options;

use crate::guilds::ServersTable;
use crate::sqlx_lib::PostgresPool;
use crate::utils::message_response;
use crate::{Error, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => unreachable!("User option is required"),
    };

    let guild_id = interaction.guild_id.ok_or_else(|| Error::MissingGuildId)?;

    let member = guild_id.member(&ctx, user).await.unwrap();

    let pool = PostgresPool::get(ctx).await;

    let artist_role_id = ServersTable::get_row(&pool, guild_id)
        .await
        .unwrap()
        .unwrap()
        .get_artist_role_id()
        .unwrap();

    member.add_role(&ctx, artist_role_id).await.unwrap();

    message_response(
        ctx,
        interaction,
        &format!("Added {} as an artist", member.display_name()),
    )
    .await
    .unwrap();

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
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
