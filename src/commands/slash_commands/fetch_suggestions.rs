use std::string::ToString;
use std::time;
use serenity::builder::CreateApplicationCommand;
use serenity::model::id::ChannelId;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{GuildChannel, ReactionType};
use serenity::prelude::Context;
use crate::utils::{edit_response_with_message, respond_with_message};

const SUGGESTION_CHANNEL_ID: u64 = 1068790374996377671;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let start_time = time::Instant::now();

    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    let suggestion_channel = ChannelId(SUGGESTION_CHANNEL_ID);

    interaction.defer(&ctx).await?;

    let active_guild_threads = guild_id.get_active_threads(&ctx).await?;
    let mut threads: Vec<GuildChannel> = Vec::new();

    for thread in active_guild_threads.threads {
        if thread.parent_id == Some(suggestion_channel) {
            threads.push(thread);
        }
    }

    for thread in suggestion_channel.get_archived_public_threads(&ctx, None, None).await?.threads {
        threads.push(thread);
    }

    let mut thread_reaction_counts = Vec::with_capacity(threads.len());

    for thread in threads {
        let reactions = match thread.reaction_users(&ctx, thread.id.0, ReactionType::Unicode("👍".to_string()), Some(100), None).await {
            Ok(reactions) => reactions,
            Err(_) => continue,
        };
        thread_reaction_counts.push((thread, reactions.len()));
    }

    thread_reaction_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let elapsed_time = start_time.elapsed();

    let result = interaction.user.dm(&ctx, |f| {
        f.embed(|e| {
            e.title("Top 10 suggestions");
            e.description("Here are the top 10 suggestions, sorted by upvotes.");

            for (i, (thread, count)) in thread_reaction_counts[..10].iter().enumerate() {
                e.field(format!("{}. 👍: {}", i + 1, count), format!("Link: {}", thread), false);
            }
            e
        })
    }).await;

    if result.is_err() {
        return respond_with_message(ctx, interaction, "I couldn't DM you. Please enable DMs from server members and try again.").await;
    }

    edit_response_with_message(ctx, interaction, &format!("Suggestions fetched. Took {} seconds", elapsed_time.as_secs())).await.expect("Error editing response");
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("fetch_suggestions").description("Fetch suggestions from the suggestion channel")
}