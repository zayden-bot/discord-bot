use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::utils::respond_with_message;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let guild = match guild_id.to_guild_cached(ctx) {
        Some(guild) => guild,
        None => return respond_with_message(ctx, interaction, "Error getting guild").await,
    };

    respond_with_message(ctx, interaction, &format!("There are **{}** members in this server", guild.member_count)).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("member_count").description("View the total member count")
}