use crate::utils::message_response;
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateMessage,
    GuildChannel, Message, ReactionType,
};
use std::time;

const SUGGESTION_CHANNEL_ID: u64 = 1068790374996377671;

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let start_time = time::Instant::now();

    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return message_response(
                &ctx,
                interaction,
                "This command can only be used in a server",
            )
            .await
        }
    };

    let suggestion_channel = ChannelId::new(SUGGESTION_CHANNEL_ID);

    let active_guild_threads = guild_id.get_active_threads(&ctx).await?;
    let mut threads: Vec<GuildChannel> = Vec::new();

    for thread in active_guild_threads.threads {
        if thread.parent_id == Some(suggestion_channel) {
            threads.push(thread);
        }
    }

    for thread in suggestion_channel
        .get_archived_public_threads(&ctx, None, None)
        .await?
        .threads
    {
        threads.push(thread);
    }

    let mut thread_reaction_counts = Vec::with_capacity(threads.len());

    for thread in threads {
        let reactions = match thread
            .reaction_users(
                &ctx,
                thread.id.get(),
                ReactionType::Unicode("👍".to_string()),
                Some(100),
                None,
            )
            .await
        {
            Ok(reactions) => reactions,
            Err(_) => continue,
        };
        thread_reaction_counts.push((thread, reactions.len()));
    }

    thread_reaction_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let elapsed_time = start_time.elapsed();

    let fields = thread_reaction_counts[..10]
        .iter()
        .enumerate()
        .map(|(i, (thread, count))| {
            (
                format!("{}. 👍: {}", i + 1, count),
                format!("Link: {}", thread),
                false,
            )
        })
        .collect::<Vec<(String, String, bool)>>();

    let result = interaction
        .user
        .dm(
            &ctx,
            CreateMessage::new().add_embed(
                CreateEmbed::new()
                    .title("Top 10 suggestions")
                    .description("Here are the top 10 suggestions, sorted by votes.")
                    .fields(fields),
            ),
        )
        .await;

    match result {
        Ok(_) => {
            message_response(
                &ctx,
                interaction,
                &format!(
                    "Suggestions fetched. Took {} seconds",
                    elapsed_time.as_secs()
                ),
            )
            .await
        }

        Err(_) => {
            message_response(
                &ctx,
                interaction,
                "I couldn't DM you. Please enable DMs from server members and try again.",
            )
            .await
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("fetch_suggestions")
        .description("Fetch suggestions from the suggestion channel")
}
