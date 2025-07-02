use serenity::all::CreateCommand;

pub mod commands;
pub mod error;
pub mod manager;

pub use error::Error;
use error::Result;
pub use manager::{GoldStarManager, GoldStarRow};

pub struct GiveStar;
pub struct Stars;

pub fn register() -> Vec<CreateCommand> {
    vec![GiveStar::register(), Stars::register()]
}
