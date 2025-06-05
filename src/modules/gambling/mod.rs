use async_trait::async_trait;
use gambling::{GameManager, GameRow};
use serenity::all::{Context, CreateCommand, UserId};
use sqlx::{PgPool, Postgres, any::AnyQueryResult};
use zayden_core::SlashCommand;

mod coinflip;
mod craft;
mod daily;
mod dig;
mod effects;
mod gift;
mod goals;
mod higher_lower;
mod inventory;
mod leaderboard;
mod lotto;
mod mine;
mod profile;
mod roll;
mod rps;
mod send;
mod shop;
mod stamina;
mod tictactoe;
mod work;

pub use coinflip::Coinflip;
pub use craft::Craft;
pub use daily::Daily;
pub use dig::Dig;
pub use effects::EffectsTable;
pub use gift::Gift;
pub use goals::{Goals, GoalsTable};
pub use higher_lower::HigherLower;
pub use inventory::Inventory;
pub use leaderboard::Leaderboard;
pub use lotto::{Lotto, LottoTable};
pub use mine::{Mine, MineTable};
pub use profile::Profile;
pub use roll::Roll;
pub use rps::RockPaperScissors;
pub use send::Send;
pub use shop::Shop;
pub use stamina::StaminaTable;
pub use tictactoe::TicTacToe;
pub use work::Work;

pub fn register(ctx: &Context) -> [CreateCommand; 18] {
    [
        Coinflip::register(ctx).unwrap(),
        Craft::register(ctx).unwrap(),
        Daily::register(ctx).unwrap(),
        Dig::register(ctx).unwrap(),
        Gift::register(ctx).unwrap(),
        Goals::register(ctx).unwrap(),
        HigherLower::register(ctx).unwrap(),
        Inventory::register(ctx).unwrap(),
        Leaderboard::register(ctx).unwrap(),
        Lotto::register(ctx).unwrap(),
        Mine::register(ctx).unwrap(),
        Profile::register(ctx).unwrap(),
        Roll::register(ctx).unwrap(),
        RockPaperScissors::register(ctx).unwrap(),
        Send::register(ctx).unwrap(),
        Shop::register(ctx).unwrap(),
        TicTacToe::register(ctx).unwrap(),
        Work::register(ctx).unwrap(),
    ]
}

pub struct GamblingTable;

impl GamblingTable {
    pub async fn add_coins(
        pool: &PgPool,
        id: impl Into<UserId>,
        amount: i64,
    ) -> sqlx::Result<AnyQueryResult> {
        let id = id.into();

        sqlx::query!(
            "UPDATE gambling SET coins = coins + $2 WHERE id = $1",
            id.get() as i64,
            amount
        )
        .execute(pool)
        .await
        .map(|r| r.into())
    }
}

pub struct GameTable;

#[async_trait]
impl GameManager<Postgres> for GameTable {
    async fn row(
        pool: &PgPool,
        id: impl Into<UserId> + std::marker::Send,
    ) -> sqlx::Result<Option<GameRow>> {
        let id = id.into();

        sqlx::query_as!(
            GameRow,
            "SELECT g.id, g.coins, g.gems, COALESCE(l.level, 0) AS level FROM gambling g LEFT JOIN levels l ON g.id = l.id WHERE g.id = $1;",
            id.get() as i64
        ).fetch_optional(pool).await
    }

    async fn save(pool: &PgPool, row: GameRow) -> sqlx::Result<AnyQueryResult> {
        sqlx::query!(
            "INSERT INTO gambling (id, coins, gems)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET
            coins = EXCLUDED.coins, gems = EXCLUDED.gems;",
            row.id,
            row.coins,
            row.gems,
        )
        .execute(pool)
        .await
        .map(AnyQueryResult::from)
    }
}
