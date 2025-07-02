use std::collections::HashMap;

use serenity::all::{
    CommandInteraction, Context, EditChannel, EditInteractionResponse, GuildId, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{Error, Result, TicketGuildManager};

use super::TicketCommand;

impl TicketCommand {
    pub(super) async fn fixed<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        mut options: HashMap<&str, ResolvedValue<'_>>,
        guild_id: GuildId,
    ) -> Result<()> {
        let version = match options.remove("version") {
            Some(ResolvedValue::String(message)) => message,
            _ => "",
        };

        match version.is_empty() {
            true => interaction.defer_ephemeral(&ctx).await?,
            false => interaction.defer(&ctx).await?,
        };

        let support_channel_id = GuildManager::get(pool, guild_id)
            .await
            .unwrap()
            .unwrap()
            .channel_id()
            .unwrap();

        let channel = interaction.channel.as_ref().unwrap();

        if channel.parent_id.unwrap() != support_channel_id {
            return Err(Error::NotInSupportChannel);
        }

        let new_channel_name = format!("{} - {}", "[Fixed]", channel.name.as_ref().unwrap())
            .chars()
            .take(100)
            .collect::<String>();

        interaction
            .channel_id
            .edit(ctx, EditChannel::new().name(new_channel_name))
            .await
            .unwrap();

        let response = if version.is_empty() {
            EditInteractionResponse::new().content("Ticket marked as fixed")
        } else {
            EditInteractionResponse::new()
                .content(format!("Ticket marked as fixed for {}", version))
        };

        interaction.edit_response(ctx, response).await?;

        Ok(())
    }
}
