use serenity::builder::CreateApplicationCommand;
use serenity::model::id::ChannelId;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::ReactionType;
use serenity::prelude::Context;
use crate::utils::{edit_response_with_message, respond_with_message};

const SUGGESTION_CHANNEL_ID: u64 = 1068790374996377671;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return respond_with_message(ctx, interaction, "This command can only be used in a server").await,
    };

    interaction.defer(&ctx).await.unwrap();

    let active_threads = guild_id.get_active_threads(&ctx).await.unwrap();
    let suggestion_threads = active_threads.threads.iter().filter(|thread| thread.parent_id == Some(ChannelId(SUGGESTION_CHANNEL_ID))).collect::<Vec<_>>();

    for thread in suggestion_threads {
        let reactions = thread.reaction_users(&ctx, thread.id.0, ReactionType::Unicode("👍".to_string()), None, None).await.unwrap();
        println!("{:?}", reactions);
    }

    edit_response_with_message(ctx, interaction, "Suggestions fetched").await.expect("Error editing response");
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("fetch_suggestions").description("Fetch suggestions from the suggestion channel")
}