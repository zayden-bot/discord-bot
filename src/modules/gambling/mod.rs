use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use serenity::all::{Context, CreateCommand, EmojiId, GuildId, UserId};
use sqlx::any::AnyQueryResult;
use sqlx::{FromRow, PgPool};
use zayden_core::SlashCommand;

mod coinflip;
mod daily;
mod gift;
mod higher_lower;
mod leaderboard;
mod profile;
mod roll;
mod rps;
mod send;
mod stats;
mod tictactoe;
mod work;

pub use coinflip::Coinflip;
pub use daily::Daily;
pub use gift::Gift;
pub use higher_lower::HigherLower;
pub use leaderboard::Leaderboard;
pub use profile::Profile;
pub use roll::Roll;
pub use rps::RPS;
pub use send::Send;
pub use stats::Stats;
pub use tictactoe::TicTacToe;
pub use work::Work;

use crate::sqlx_lib::GuildTable;
use crate::{Error, Result};

const START_AMOUNT: i64 = 1000;
const COIN: EmojiId = EmojiId::new(1356741391090454705);

pub fn register(ctx: &Context) -> Vec<CreateCommand> {
    vec![
        Coinflip::register(ctx).unwrap(),
        Daily::register(ctx).unwrap(),
        Gift::register(ctx).unwrap(),
        HigherLower::register(ctx).unwrap(),
        Leaderboard::register(ctx).unwrap(),
        Profile::register(ctx).unwrap(),
        Roll::register(ctx).unwrap(),
        RPS::register(ctx).unwrap(),
        Send::register(ctx).unwrap(),
        Stats::register(ctx).unwrap(),
        TicTacToe::register(ctx).unwrap(),
        Work::register(ctx).unwrap(),
    ]
    /*
    blackjack
    buy
    connectfour
    crash
    dig
    findthelady
    gamble
    lotto
    poker
    process
    race
    roulette
    sevens
    slots
    spin

    oscar can we get bank robberies that get diff chances and payout depending on how many people and pot

    Should be possible honestly, a redeem on twitch to add coins here, just not the other way around
    */
}

pub struct GamblingTable;

impl GamblingTable {
    async fn get(pool: &PgPool, id: impl Into<UserId>) -> sqlx::Result<Option<GamblingRow>> {
        let id = id.into().get() as i64;

        let row = sqlx::query_as!(GamblingRow, "SELECT * FROM gambling WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;

        Ok(row)
    }

    async fn guild_leaderboard(pool: &PgPool, page: i64) -> sqlx::Result<Vec<GamblingRow>> {
        const LIMIT: i64 = 10;

        let offset = (page - 1) * LIMIT;

        let data = sqlx::query_as!(
            GamblingRow,
            "SELECT * FROM gambling ORDER BY cash DESC  LIMIT 10 OFFSET $1",
            offset
        )
        .fetch_all(pool)
        .await
        .unwrap();

        Ok(data)
    }

    pub async fn add_cash(
        pool: &PgPool,
        id: impl Into<UserId>,
        amount: i64,
    ) -> sqlx::Result<AnyQueryResult> {
        let id = id.into().get() as i64;

        let r = sqlx::query!(
            "UPDATE gambling SET cash = cash + $2 WHERE id = $1",
            id,
            amount
        )
        .execute(pool)
        .await?;

        Ok(r.into())
    }

    async fn save(pool: &PgPool, row: GamblingRow) -> sqlx::Result<GamblingRow> {
        sqlx::query!(
            "INSERT INTO gambling (id, cash, daily, work, gift, game)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id)
            DO UPDATE SET cash = EXCLUDED.cash,
                          daily = EXCLUDED.daily,
                          work = EXCLUDED.work,
                          gift = EXCLUDED.gift,
                          game = EXCLUDED.game;",
            row.id,
            row.cash,
            row.daily,
            row.work,
            row.gift,
            row.game
        )
        .execute(pool)
        .await?;

        Ok(row)
    }
}

#[derive(FromRow)]
pub struct GamblingRow {
    pub id: i64,
    pub cash: i64,
    pub daily: NaiveDate,
    pub work: NaiveDateTime,
    pub gift: NaiveDate,
    pub game: NaiveDateTime,
}

impl GamblingRow {
    pub fn new(id: impl Into<UserId>) -> Self {
        Self {
            id: id.into().get() as i64,
            cash: START_AMOUNT,
            daily: NaiveDate::default(),
            work: NaiveDateTime::default(),
            gift: NaiveDate::default(),
            game: NaiveDateTime::default(),
        }
    }

    pub async fn from_table(pool: &PgPool, id: impl Into<UserId>) -> sqlx::Result<Self> {
        let id = id.into();

        GamblingTable::get(pool, id)
            .await
            .map(|row| row.unwrap_or_else(|| Self::new(id)))
    }

    pub fn user_id(&self) -> UserId {
        UserId::new(self.id as u64)
    }

    pub fn add_cash(&mut self, payout: i64) -> i64 {
        self.cash += payout;
        self.cash
    }

    pub fn update_game(&mut self) {
        self.game = Utc::now().naive_utc();
    }
}

impl GuildTable {
    async fn get(pool: &PgPool, id: impl Into<GuildId>) -> sqlx::Result<Option<GamblingGuildRow>> {
        let id = id.into().get() as i64;

        let row = sqlx::query_as!(
            GamblingGuildRow,
            "SELECT id, gambling_lost, gambling_gain FROM guilds WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await
        .unwrap();

        Ok(row)
    }

    async fn update_stats(
        pool: &PgPool,
        id: impl Into<GuildId>,
        amount: i64,
    ) -> sqlx::Result<AnyQueryResult> {
        let id = id.into().get() as i64;

        let (gain, loss) = if amount > 0 { (amount, 0) } else { (0, amount) };

        let r = sqlx::query!("UPDATE guilds SET gambling_lost = gambling_lost + $2, gambling_gain = gambling_gain + $3 WHERE id = $1", id, loss, gain).execute(pool).await?;

        Ok(r.into())
    }
}

#[derive(FromRow)]
pub struct GamblingGuildRow {
    pub id: i64,
    pub gambling_lost: i64,
    pub gambling_gain: i64,
}

pub struct GamblingAndLevelsRow {
    pub id: i64,
    pub cash: i64,
    pub daily: NaiveDate,
    pub work: NaiveDateTime,
    pub gift: NaiveDate,
    pub game: NaiveDateTime,
    pub total_xp: i32,
    pub last_xp: NaiveDateTime,
    pub xp: i32,
    pub level: i32,
    pub message_count: i32,
}

impl GamblingAndLevelsRow {
    pub async fn from_table(pool: &PgPool, id: impl Into<UserId>) -> Self {
        let id = id.into().get() as i64;

        sqlx::query_as!(
            GamblingAndLevelsRow,
            "SELECT g.*, l.total_xp, l.last_xp, l.xp, l.level, l.message_count FROM gambling g LEFT JOIN levels l ON g.id = l.id WHERE g.id = $1",
            id
        ).fetch_one(pool).await.unwrap()
    }
}

fn verify_cooldown(row: &GamblingRow) -> Result<()> {
    let cooldown_over = row.game + Duration::seconds(4);

    if cooldown_over >= Utc::now().naive_utc() {
        return Err(Error::cooldown(cooldown_over.and_utc().timestamp()));
    }

    Ok(())
}

fn verify_bet(row: &GamblingRow, bet: i64) -> Result<()> {
    const MIN_AMOUNT: i64 = 1;

    if bet < MIN_AMOUNT {
        return Err(Error::invalid_bet_amount(MIN_AMOUNT));
    }

    if bet > row.cash {
        return Err(Error::InsufficientFunds);
    }

    Ok(())
}
