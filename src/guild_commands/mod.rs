use serenity::all::Context;

use crate::Result;

pub mod college_kings;
pub mod college_kings_team;

pub async fn register(ctx: &Context) -> Result<()> {
    tokio::try_join!(
        college_kings::register(ctx),
        college_kings_team::register(ctx)
    )?;

    println!("Guild commands registered!");

    Ok(())
}
