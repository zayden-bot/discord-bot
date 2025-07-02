use std::collections::HashMap;

use serenity::all::{
    CommandInteraction, Context, EditInteractionResponse, MessageId, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{ticket_manager::TicketManager, Result};

use super::TicketCommand;

impl TicketCommand {
    pub(super) async fn remove<Db: Database, Manager: TicketManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        mut options: HashMap<&str, ResolvedValue<'_>>,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        let message_id = match options.remove("message") {
            Some(ResolvedValue::Integer(id)) => MessageId::new(id as u64),
            _ => unreachable!("ID is required"),
        };

        let channel_id = match options.remove("channel") {
            Some(ResolvedValue::Channel(channel)) => channel.id,
            _ => interaction.channel_id,
        };

        channel_id.delete_message(ctx, message_id).await.unwrap();

        Manager::delete(pool, message_id).await.unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Message removed"),
            )
            .await
            .unwrap();

        Ok(())
    }
}
