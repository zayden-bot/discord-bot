pub mod commands;
pub use commands::Commands;

pub mod message_create;
pub use message_create::message_create;

pub mod sqlx_lib;
pub use sqlx_lib::{FullLevelRow, LeaderboardRow, LevelsManager, LevelsRow, RankRow, XpRow};

#[inline(always)]
pub const fn level_up_xp(level: i32) -> i32 {
    (3 * level * level) + (50 * level) + 100
}
