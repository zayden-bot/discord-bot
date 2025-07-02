use std::collections::HashMap;

use serenity::all::{
    CommandInteraction, Context, EditChannel, EditInteractionResponse, GuildId, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{Error, Result, TicketGuildManager};

use super::TicketCommand;

impl TicketCommand {
    pub(super) async fn close<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        mut options: HashMap<&str, ResolvedValue<'_>>,
        guild_id: GuildId,
    ) -> Result<()> {
        let message = match options.remove("message") {
            Some(ResolvedValue::String(message)) => message,
            _ => "",
        };

        match message.is_empty() {
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

        let new_channel_name: String =
            format!("{} - {}", "[Closed]", channel.name.as_ref().unwrap())
                .chars()
                .take(100)
                .collect();

        interaction
            .channel_id
            .edit(&ctx, EditChannel::new().name(new_channel_name))
            .await
            .unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content(format!("Ticket marked as closed\n\n{}", message)),
            )
            .await
            .unwrap();

        Ok(())
    }
}
