use serenity::all::{Context, ModalInteraction};
use sqlx::Postgres;

use crate::sqlx_lib::PostgresPool;
use crate::Result;

use super::{LfgGuildTable, LfgPostTable, UsersTable};

pub struct LfgCreateModal;

impl LfgCreateModal {
    pub async fn run(ctx: &Context, interaction: &ModalInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::LfgCreateModal::run::<Postgres, LfgGuildTable, LfgPostTable, UsersTable>(
            ctx,
            interaction,
            &pool,
        )
        .await?;

        Ok(())
    }
}

pub struct LfgEditModal;

impl LfgEditModal {
    pub async fn run(ctx: &Context, interaction: &ModalInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::LfgEditModal::run::<Postgres, LfgPostTable, UsersTable>(ctx, interaction, &pool)
            .await?;

        Ok(())
    }
}
