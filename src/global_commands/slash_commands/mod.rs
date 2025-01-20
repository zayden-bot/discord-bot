use serenity::all::{Context, CreateCommand, Ready};
use zayden_core::SlashCommand;

use crate::Result;

pub mod member_count;
pub mod ping;
pub mod scam;
pub mod server_info;

pub use member_count::MemberCount;
pub use ping::Ping;
pub use scam::Scam;
pub use server_info::ServerInfo;

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![
        MemberCount::register(ctx, ready)?,
        Ping::register(ctx, ready)?,
        Scam::register(ctx, ready)?,
        ServerInfo::register(ctx, ready)?,
    ];

    Ok(commands)
}
