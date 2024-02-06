use crate::utils::{message_response, send_message};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, Message,
    Permissions,
};

const ARTIST_ROLE_ID: u64 = 1043987303556726854;

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return message_response(
                &ctx,
                interaction,
                "This command can only be used in a server",
            )
            .await
        }
    };

    let user_id = match interaction.data.options[0].value.as_user_id() {
        Some(user) => user,
        None => return message_response(&ctx, interaction, "Please provide a valid user").await,
    };

    let member = match guild_id.member(&ctx, user_id).await {
        Ok(member) => member,
        Err(_) => return message_response(&ctx, interaction, "Error retrieving member").await,
    };

    match member.add_role(&ctx, ARTIST_ROLE_ID).await {
        Ok(_) => {
            send_message(
                &ctx,
                interaction,
                &format!("Added {} as an artist", member.display_name()),
            )
            .await
        }
        Err(_) => message_response(&ctx, interaction, "Error adding role to member").await,
    }
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
