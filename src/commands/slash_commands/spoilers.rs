use crate::sqlx_lib::{get_spoiler_channel_ids, get_support_channel_ids};
use crate::utils::respond_with_message;
use serenity::all::{CommandInteraction, Context, CreateCommand};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
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

    let support_thread_ids = match get_support_channel_ids(guild_id.get() as i64).await {
        Ok(support_thread_ids) => support_thread_ids,
        Err(_) => {
            return respond_with_message(ctx, interaction, "Error retrieving support channel").await
        }
    };

    let support_thread_id = match support_thread_ids.first() {
        Some(support_thread_id) => support_thread_id,
        None => {
            return respond_with_message(ctx, interaction, "Error retrieving support channel").await
        }
    };

    let spoiler_thread_ids = match get_spoiler_channel_ids(guild_id.get() as i64).await {
        Ok(support_thread_ids) => support_thread_ids,
        Err(_) => {
            return respond_with_message(ctx, interaction, "Error retrieving spoiler channel").await
        }
    };

    let spoiler_thread_id = match spoiler_thread_ids.first() {
        Some(support_thread_id) => support_thread_id,
        None => {
            return respond_with_message(ctx, interaction, "Error retrieving spoiler channel").await
        }
    };

    respond_with_message(ctx, interaction, &format!("Please keep all conversations about the new update to <#{}>\nIf you have any bugs or questions please post them in <#{}>", spoiler_thread_id, support_thread_id)).await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("spoilers").description("Disclaimer about spoilers")
}
