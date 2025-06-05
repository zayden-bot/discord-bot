use async_trait::async_trait;
use gambling::{Commands, MineManager, mine::MineRow};
use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption, UserId};
use sqlx::{PgPool, Postgres, any::AnyQueryResult};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct MineTable;

#[async_trait]
impl MineManager<Postgres> for MineTable {
    async fn row(pool: &PgPool, id: impl Into<UserId> + Send) -> sqlx::Result<Option<MineRow>> {
        let id = id.into();

        sqlx::query_as!(
            MineRow,
            "SELECT miners, mines, land, countries, continents, planets, solar_systems, galaxies, universes, prestige FROM gambling_mine WHERE id = $1",
            id.get() as i64
        ).fetch_optional(pool).await
    }

    async fn add_coins(pool: &PgPool) -> sqlx::Result<AnyQueryResult> {
        sqlx::query!(
            "UPDATE gambling g
            SET coins = g.coins + m.miners * 10
            FROM gambling_mine m
            WHERE g.id = m.id;"
        )
        .execute(pool)
        .await
        .map(AnyQueryResult::from)
    }
}

pub struct Mine;

#[async_trait]
impl SlashCommand<Error, Postgres> for Mine {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        Commands::mine::<Postgres, MineTable>(ctx, interaction, pool).await?;

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(Commands::register_mine())
    }
}
