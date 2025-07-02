use std::collections::HashMap;

use serenity::all::{
    ChannelType, CommandInteraction, Context, CreateChannel, EditInteractionResponse, GuildId,
    ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{guild_manager::TempVoiceGuildManager, Result};

pub async fn setup<Db: Database, Manager: TempVoiceGuildManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    guild_id: GuildId,
    mut options: HashMap<&str, ResolvedValue<'_>>,
) -> Result<()> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    if !interaction
        .member
        .as_ref()
        .unwrap()
        .permissions
        .unwrap()
        .administrator()
    {
        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content("You must be an administrator to run this command."),
            )
            .await
            .unwrap();
        return Ok(());
    }

    let category = match options.remove("category") {
        Some(ResolvedValue::Channel(category)) => category,
        _ => unreachable!("Category is required"),
    };

    let creator_channel = guild_id
        .create_channel(
            ctx,
            CreateChannel::new("➕ Creator Channel")
                .category(category.id)
                .kind(ChannelType::Voice),
        )
        .await
        .unwrap();

    Manager::save(pool, guild_id, category.id, creator_channel.id)
        .await
        .unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Setup complete."),
        )
        .await
        .unwrap();

    Ok(())
}
