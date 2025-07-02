use async_trait::async_trait;
use sqlx::any::AnyQueryResult;
use sqlx::{Database, Pool};

#[async_trait]
pub trait TableRow {
    async fn save<Db: Database>(&self, _pool: &Pool<Db>) -> sqlx::Result<AnyQueryResult> {
        unimplemented!()
    }
}
