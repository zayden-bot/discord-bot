use futures::StreamExt;
use serenity::all::{
    CommandInteraction, Context, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
    EditInteractionResponse, GuildId,
};
use sqlx::{Database, Pool};

use crate::{Result, TicketGuildManager};

use super::SupportCommand;

impl SupportCommand {
    pub(super) async fn list<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        guild_id: GuildId,
    ) -> Result<()> {
        let faq_channel_id = GuildManager::get(pool, guild_id)
            .await
            .unwrap()
            .unwrap()
            .faq_channel_id()
            .unwrap();

        let menu_options = faq_channel_id
            .messages_iter(ctx)
            .enumerate()
            .map(|(index, msg_result)| {
                let msg = msg_result.unwrap();
                let id = msg.content.lines().next().unwrap().trim();

                CreateSelectMenuOption::new(id[2..id.len() - 2].to_string(), index.to_string())
            })
            .collect::<Vec<_>>()
            .await;

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().select_menu(CreateSelectMenu::new(
                    "support_faq",
                    CreateSelectMenuKind::String {
                        options: menu_options,
                    },
                )),
            )
            .await
            .unwrap();

        Ok(())
    }
}
