use serenity::all::{CommandInteraction, Context, EditChannel, EditInteractionResponse, GuildId};
use sqlx::{Database, Pool};

use crate::{Error, Result, TicketGuildManager};

use super::TicketCommand;

impl TicketCommand {
    pub(super) async fn open<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        guild_id: GuildId,
    ) -> Result<()> {
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

        let new_channel_name = channel
            .name
            .as_ref()
            .unwrap()
            .replace("[Fixed] - ", "")
            .replace("[Closed] - ", "");

        interaction
            .channel_id
            .edit(&ctx, EditChannel::new().name(new_channel_name))
            .await
            .unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Ticket reopened"),
            )
            .await
            .unwrap();

        Ok(())
    }
}
