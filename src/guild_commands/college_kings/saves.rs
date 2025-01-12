use serenity::all::{CommandInteraction, Context, CreateCommand, Ready};

use crate::guilds::ServersTable;
use crate::sqlx_lib::PostgresPool;
use crate::utils::message_response;
use crate::{Error, Result};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.ok_or_else(|| Error::NotInGuild)?;

    let pool = PostgresPool::get(ctx).await;

    let support_thread_id = ServersTable::get_row(&pool, guild_id)
        .await
        .unwrap()
        .unwrap()
        .get_support_channel_id()
        .unwrap();

    message_response(ctx, interaction, format!("We do our best to retain save integrity with every update however due to the dynamic nature of game development saves might break. If you experience a save problem please let us know in <#{}>", support_thread_id)).await.unwrap();

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("saves").description("Get saves disclaimer");

    Ok(command)
}
