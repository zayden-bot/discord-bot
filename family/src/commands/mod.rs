use async_trait::async_trait;
use serenity::all::{CommandInteraction, Context, CreateCommand};
use sqlx::{Database, Pool};

use crate::family_manager::FamilyManager;
use crate::Result;

mod adopt;
mod block;
// mod cache;
mod information;
mod marry;
mod moderation;
mod tree;

pub use adopt::Adopt;
pub use block::{Block, Unblock};
pub use information::Children;
pub use information::Parents;
pub use information::Partner;
pub use information::Relationship;
pub use information::Siblings;
pub use marry::Marry;
pub use tree::Tree;

#[async_trait]
pub trait FamilyCommand<T> {
    async fn run<Db: Database, Manager: FamilyManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
    ) -> Result<T>;

    fn register() -> CreateCommand;
}
