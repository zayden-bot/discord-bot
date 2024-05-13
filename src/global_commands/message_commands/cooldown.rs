use serenity::all::{Context, CreateMessage, Mentionable, Message};

use crate::{state::State, Result};

pub async fn run(ctx: &Context, msg: &Message) -> Result<()> {
    if msg.guild_id.is_none() {
        return Ok(());
    }

    let data = ctx.data.read().await;
    let state = data
        .get::<State>()
        .ok_or_else(|| crate::Error::DataNotFound)?;

    for (key, condition) in &state.cooldown_conditions {
        if condition(&msg.content) {
            msg.delete(ctx).await?;
            msg.channel_id
                .send_message(
                    ctx,
                    CreateMessage::new().content(format!(
                        "{} By orders of the highest authority, {} are currently on cooldown.",
                        msg.author.mention(),
                        key
                    )),
                )
                .await?;
        }
    }

    Ok(())
}
