use crate::sqlx_lib::{get_support_channel_ids, PostgresPool};
use crate::utils::message_response;
use crate::{Error, Result};
use serenity::all::{CommandInteraction, Context, CreateCommand};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NoGuild)?;

    let pool = {
        let data = ctx.data.read().await;
        data.get::<PostgresPool>()
            .expect("PostgresPool should exist in data.")
            .clone()
    };

    let support_thread_ids = get_support_channel_ids(&pool, guild_id.get() as i64).await?;
    let support_thread_id = support_thread_ids
        .first()
        .ok_or_else(|| Error::NoSupportThread)?;

    message_response(ctx, interaction, format!("We do our best to retain save integrity with every update however due to the dynamic nature of game development saves might break. If you experience a save problem please let us know in <#{}>", support_thread_id)).await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("saves").description("Get saves disclaimer")
}
