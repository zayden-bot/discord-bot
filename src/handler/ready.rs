use gambling::StaminaCron;
use gambling::commands::Lotto;
use gambling::mine::Mine;
use lfg::close_old_posts;
use serenity::all::{Context, OnlineStatus, Ready};
use sqlx::{PgPool, Postgres};

use crate::Result;
use crate::cron::start_cron_jobs;
use crate::handler::Handler;
use crate::modules::destiny2::lfg::LfgPostTable;
use crate::modules::gambling::{LottoTable, MineTable, StaminaTable};

impl Handler {
    pub async fn ready(ctx: &Context, ready: Ready, pool: &PgPool) -> Result<()> {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(None, OnlineStatus::Online);

        close_old_posts::<Postgres, LfgPostTable>(ctx, pool).await;

        let ctx = ctx.clone();
        let pool = pool.clone();

        tokio::spawn(async move {
            start_cron_jobs(
                ctx,
                pool,
                vec![
                    Lotto::cron_job::<Postgres, LottoTable>(),
                    StaminaCron::cron_job::<Postgres, StaminaTable>(),
                    Mine::cron_job::<Postgres, MineTable>(),
                ],
            )
            .await
        });

        Ok(())
    }
}
