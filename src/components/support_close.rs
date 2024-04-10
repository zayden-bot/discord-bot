use serenity::all::{
    Channel, ComponentInteraction, Context, CreateInteractionResponse, EditThread,
};

use crate::Result;

pub async fn support_close(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let mut channel = match interaction.channel_id.to_channel(ctx).await? {
        Channel::Guild(channel) => channel,
        _ => unreachable!("Support close button can only be used in guild channels"),
    };

    let new_channel_name: String = format!("{} - {}", "[Closed]", channel.name)
        .chars()
        .take(100)
        .collect();

    channel
        .edit_thread(ctx, EditThread::new().name(new_channel_name).archived(true))
        .await?;

    interaction
        .create_response(ctx, CreateInteractionResponse::Acknowledge)
        .await?;

    Ok(())
}
