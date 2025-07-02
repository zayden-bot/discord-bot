use endgame_analysis::slash_commands::{DimWishlist, TierList, Weapon};
use info::Perk;
use serenity::all::{Context, CreateCommand};
use zayden_core::SlashCommand;

pub mod endgame_analysis;
pub mod info;

pub fn register(ctx: &Context) -> [CreateCommand; 4] {
    [
        DimWishlist::register(ctx).unwrap(),
        Weapon::register(ctx).unwrap(),
        TierList::register(ctx).unwrap(),
        Perk::register(ctx).unwrap(),
    ]
}
