use std::collections::HashMap;

use serenity::all::{
    CommandInteraction, Context, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
    EditInteractionResponse, GuildChannel, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{Error, PostManager, Result};

use super::Command;

impl Command {
    pub async fn tags<Db: Database, Manager: PostManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        options: HashMap<&str, ResolvedValue<'_>>,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let post_owner = Manager::owner(pool, interaction.channel_id).await.unwrap();

        if post_owner != interaction.user.id {
            return Err(Error::PermissionDenied(post_owner));
        }

        let thread_channel = interaction
            .channel_id
            .to_channel(ctx)
            .await
            .unwrap()
            .guild()
            .unwrap();

        let forum_channel = thread_channel
            .parent_id
            .unwrap()
            .to_channel(ctx)
            .await
            .unwrap()
            .guild()
            .unwrap();

        if options.contains_key("add") {
            add_tags(ctx, interaction, forum_channel, thread_channel)
                .await
                .unwrap();
        } else if options.contains_key("remove") {
            remove_tags(ctx, interaction, forum_channel, thread_channel)
                .await
                .unwrap();
        }

        Ok(())
    }
}
async fn add_tags(
    ctx: &Context,
    interaction: &CommandInteraction,
    forum_channel: GuildChannel,
    thread_channel: GuildChannel,
) -> Result<()> {
    let options = forum_channel
        .available_tags
        .into_iter()
        .filter(|tag| !thread_channel.applied_tags.contains(&tag.id))
        .map(|tag| CreateSelectMenuOption::new(tag.name, tag.id.to_string()))
        .collect::<Vec<_>>();

    let max_values = options.len() as u8;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().select_menu(
                CreateSelectMenu::new("lfg_tags_add", CreateSelectMenuKind::String { options })
                    .max_values(max_values),
            ),
        )
        .await
        .unwrap();

    Ok(())
}

async fn remove_tags(
    ctx: &Context,
    interaction: &CommandInteraction,
    forum_channel: GuildChannel,
    thread_channel: GuildChannel,
) -> Result<()> {
    let options = forum_channel
        .available_tags
        .into_iter()
        .filter(|tag| thread_channel.applied_tags.contains(&tag.id))
        .map(|tag| CreateSelectMenuOption::new(tag.name, tag.id.to_string()))
        .collect::<Vec<_>>();

    let max_values = options.len() as u8;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().select_menu(
                CreateSelectMenu::new("lfg_tags_remove", CreateSelectMenuKind::String { options })
                    .max_values(max_values),
            ),
        )
        .await
        .unwrap();

    Ok(())
}
