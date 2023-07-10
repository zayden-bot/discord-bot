use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::sqlx_lib::get_support_channel_ids;
use crate::utils::respond_with_message;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let support_thread_ids = match get_support_channel_ids(guild_id.0 as i64).await {
        Ok(support_thread_ids) => support_thread_ids,
        Err(_) => return respond_with_message(ctx, interaction, "Error retrieving support channel").await,
    };

    let support_thread_id = match support_thread_ids.first() {
        Some(support_thread_id) => support_thread_id,
        None => return respond_with_message(ctx, interaction, "Error retrieving support channel").await,
    };

    respond_with_message(ctx, interaction, &format!("We do our best to retain save integrity with every update however due to the dynamic nature of game development saves might break. If you experience a save problem please let us know in <#{}>", support_thread_id)).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("saves").description("Get saves disclaimer")
}