use crate::{guilds::college_kings::SUGGESTION_CATEGORY_ID, utils::message_response};
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed, CreateMessage, GuildChannel,
    ReactionType, Ready,
};
use std::time;

use crate::Result;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let start_time = time::Instant::now();

    let guild_id = interaction.guild_id.unwrap();

    let active_guild_threads = guild_id.get_active_threads(&ctx).await.unwrap();
    let threads: Vec<GuildChannel> = active_guild_threads
        .threads
        .into_iter()
        .chain(
            SUGGESTION_CATEGORY_ID
                .get_archived_public_threads(&ctx, None, None)
                .await
                .unwrap()
                .threads,
        )
        .filter(|thread| {
            thread
                .parent_id
                .is_some_and(|id| id == SUGGESTION_CATEGORY_ID)
        })
        .collect();

    let mut thread_reaction_counts = Vec::with_capacity(threads.len());

    for thread in threads {
        let reactions = thread
            .reaction_users(
                &ctx,
                thread.id.get(),
                ReactionType::Unicode("👍".to_string()),
                Some(100),
                None,
            )
            .await
            .unwrap();
        thread_reaction_counts.push((thread, reactions.len()));
    }

    thread_reaction_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let elapsed_time = start_time.elapsed();

    let mut embed = CreateEmbed::new()
        .title("Top 10 suggestions")
        .description("Here are the top 10 suggestions, sorted by votes.");

    for (i, (thread, count)) in thread_reaction_counts[..10].iter().enumerate() {
        embed = embed.field(
            format!("{}. 👍: {}", i + 1, count),
            format!("Link: {}", thread),
            false,
        );
    }

    interaction
        .user
        .dm(&ctx, CreateMessage::new().add_embed(embed))
        .await
        .unwrap();

    message_response(
        ctx,
        interaction,
        &format!(
            "Suggestions fetched. Took {} seconds",
            elapsed_time.as_secs()
        ),
    )
    .await
    .unwrap();

    Ok(())
}

pub fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
    let command = CreateCommand::new("fetch_suggestions")
        .description("Fetch suggestions from the suggestion channel");

    Ok(command)
}
