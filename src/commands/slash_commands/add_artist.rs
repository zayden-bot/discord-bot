use crate::utils::respond_with_message;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::Permissions;
use serenity::prelude::Context;

const ARTIST_ROLE_ID: u64 = 1043987303556726854;

pub async fn run(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
    // ) -> Result<(), serenity::Error> {

    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return respond_with_message(
                ctx,
                interaction,
                "This command can only be used in a server",
            )
            .await
        }
    };

    let user = match interaction.data.options[0].resolved.as_ref() {
        Some(CommandDataOptionValue::User(user, _member)) => user,
        _ => return respond_with_message(ctx, interaction, "Please provide a valid user").await,
    };

    let mut member = match guild_id.member(ctx, user.id).await {
        Ok(member) => member,
        Err(_) => return respond_with_message(ctx, interaction, "Error retrieving member").await,
    };

    match member.add_role(ctx, ARTIST_ROLE_ID).await {
        Ok(_) => {
            respond_with_message(ctx, interaction, &format!("Added {} as an artist", user)).await
        }
        Err(_) => respond_with_message(ctx, interaction, "Error adding role to member").await,
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add_artist")
        .description("Adds a user as an artist")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .create_option(|option| {
            option
                .name("user")
                .description("The user to add as an artist")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
