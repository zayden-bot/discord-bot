use async_trait::async_trait;
use sqlx::{Database, Pool, any::AnyQueryResult};
use zayden_core::CronJob;

#[async_trait]
pub trait StaminaManager<Db: Database> {
    async fn update(pool: &Pool<Db>) -> sqlx::Result<AnyQueryResult>;
}

pub struct StaminaCron;

impl StaminaCron {
    pub fn cron_job<Db: Database, Manager: StaminaManager<Db>>() -> CronJob<Db> {
        CronJob::new("0 */10 * * * * *").set_action(|_ctx, pool| async move {
            Manager::update(&pool).await.unwrap();
        })
    }
}
