use serenity::all::Context;

use crate::guilds::college_kings_team::GUILD_ID;
use crate::Result;

pub mod review;

pub async fn register(ctx: &Context) -> Result<()> {
    GUILD_ID.set_commands(ctx, vec![review::register()]).await?;
    Ok(())
}
