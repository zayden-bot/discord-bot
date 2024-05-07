use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    Permissions, ResolvedValue,
};

use crate::{
    guilds::college_kings::ARTIST_ROLE_ID,
    utils::{message_response, parse_options},
};
use crate::{Error, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let options = interaction.data.options();
    let options = parse_options(&options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => unreachable!("User option is required"),
    };

    let guild_id = interaction.guild_id.ok_or_else(|| Error::NotInGuild)?;

    let member = guild_id.member(&ctx, user).await?;

    member.add_role(&ctx, ARTIST_ROLE_ID).await?;

    message_response(
        ctx,
        interaction,
        &format!("Added {} as an artist", member.display_name()),
    )
    .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("add_artist")
        .description("Adds a user as an artist")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to add as an artist",
            )
            .required(true),
        )
}
