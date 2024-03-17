use cron::Schedule;
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::time::sleep_until;

use crate::sqlx_lib;

pub async fn start_cron_jobs() {
    let schedule = Schedule::from_str("0 0 * * * * *").unwrap();
    let mut next = Instant::now();

    loop {
        if let Some(when) = schedule.upcoming(chrono::Utc).next() {
            let now = chrono::Utc::now();
            if now >= when {
                let _ = tokio::join!(create_total_xp_index());

                next += Duration::from_secs(60);
                sleep_until(next.into()).await;
            }
        }
    }
}

async fn create_total_xp_index() -> Result<(), sqlx::Error> {
    let pool = sqlx_lib::get_pool().await;
    let mut transaction = pool.begin().await?;

    sqlx::query("CREATE INDEX idx_total_xp_new ON levels (total_xp)")
        .execute(&mut *transaction)
        .await?;

    sqlx::query("DROP INDEX IF EXISTS idx_total_xp")
        .execute(&mut *transaction)
        .await?;

    sqlx::query("ALTER INDEX idx_total_xp_new RENAME TO idx_total_xp")
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}
