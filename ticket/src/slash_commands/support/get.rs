use std::collections::HashMap;

use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    CommandInteraction, Context, CreateEmbed, EditInteractionResponse, GuildId, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{Result, TicketGuildManager};

use super::SupportCommand;

impl SupportCommand {
    pub(super) async fn get<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        mut options: HashMap<&str, ResolvedValue<'_>>,
        guild_id: GuildId,
    ) -> Result<()> {
        interaction.defer(ctx).await?;

        let id = match options.remove("id") {
            Some(ResolvedValue::String(id)) => id,
            _ => unreachable!("ID is required"),
        };

        let faq_channel_id = GuildManager::get(pool, guild_id)
            .await
            .unwrap()
            .unwrap()
            .faq_channel_id()
            .unwrap();

        let mut stream = faq_channel_id.messages_iter(ctx).boxed();

        while let Some(msg) = stream.try_next().await.unwrap() {
            let support_id = msg.content.lines().next().unwrap().trim();

            let title = &support_id[2..support_id.len() - 2];
            let description = msg.content.strip_prefix(support_id).unwrap();

            if support_id.contains(id) {
                interaction
                    .edit_response(
                        ctx,
                        EditInteractionResponse::new()
                            .embed(CreateEmbed::new().title(title).description(description)),
                    )
                    .await
                    .unwrap();

                return Ok(());
            }
        }

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Support message not found"),
            )
            .await?;

        Ok(())
    }
}
