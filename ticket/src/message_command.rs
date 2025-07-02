use futures::{stream, StreamExt};
use serenity::all::{
    AutoArchiveDuration, ChannelType, Context, CreateAttachment, CreateEmbed, CreateMessage,
    CreateThread, DiscordJsonError, ErrorResponse, HttpError, Mentionable, Message,
};
use sqlx::{Database, Pool};

use crate::{send_support_message, thread_name, Result, TicketGuildManager};

pub struct SupportMessageCommand;

impl SupportMessageCommand {
    pub async fn run<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        message: &Message,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let Some(guild_id) = message.guild_id else {
            return Ok(());
        };

        let Some(row) = GuildManager::get(pool, guild_id).await.unwrap() else {
            return Ok(());
        };

        let Some(channel_id) = row.channel_id() else {
            return Ok(());
        };

        if channel_id != message.channel_id {
            return Ok(());
        }

        let role_ids = row.role_ids();

        let thread_name = thread_name(
            row.thread_id,
            message.author.display_name(),
            &message.content,
        );

        let thread = message
            .channel_id
            .create_thread(
                &ctx,
                CreateThread::new(thread_name)
                    .kind(ChannelType::PrivateThread)
                    .auto_archive_duration(AutoArchiveDuration::OneWeek),
            )
            .await
            .unwrap();

        GuildManager::update_thread_id(pool, guild_id)
            .await
            .unwrap();

        let issue = CreateEmbed::new()
            .title("Issue")
            .description(&message.content);

        let attachments = stream::iter(message.attachments.iter())
            .then(|attachment| async move {
                CreateAttachment::bytes(
                    attachment.download().await.unwrap(),
                    attachment.filename.clone(),
                )
            })
            .collect::<Vec<_>>()
            .await;

        let mentions = if role_ids.is_empty() {
            let owner_id = guild_id.to_partial_guild(ctx).await.unwrap().owner_id;
            vec![message.author.mention(), owner_id.mention()]
        } else {
            role_ids
                .into_iter()
                .map(|id| id.mention())
                .chain([message.author.mention()])
                .collect::<Vec<_>>()
        };

        send_support_message(
            ctx,
            thread.id,
            &mentions,
            vec![CreateMessage::new().embed(issue).files(attachments)],
        )
        .await
        .unwrap();

        match message.delete(&ctx).await {
            // 10008: Unknown Message
            Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 10008, .. },
                ..
            }))) => {}
            result => {
                result.unwrap();
            }
        }

        Ok(())
    }
}
