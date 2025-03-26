use endgame_analysis::slash_commands::{DimWishlist, TierList, Weapon};
use info::Perk;
use lfg::LfgCommand;
use serenity::all::{Context, CreateCommand, Ready};
use zayden_core::SlashCommand;

pub mod endgame_analysis;
pub mod info;
pub mod lfg;

pub fn register(ctx: &Context, ready: &Ready) -> [CreateCommand; 5] {
    [
        DimWishlist::register(ctx, ready).unwrap(),
        Weapon::register(ctx, ready).unwrap(),
        TierList::register(ctx, ready).unwrap(),
        Perk::register(ctx, ready).unwrap(),
        LfgCommand::register(ctx, ready).unwrap(),
    ]
}
