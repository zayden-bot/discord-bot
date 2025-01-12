use std::env;

use patreon_api::patreon_client::PatreonClientBuilder;
use patreon_api::types::includes::MemberInclude;
use patreon_api::types::response::MemberResponse;
use sqlx::prelude::FromRow;
use sqlx::PgPool;

use crate::Result;

use super::cache::PATREON_CLIENT_ID;

#[allow(dead_code)]
#[derive(FromRow)]
pub struct PatreonCacheRow {
    pub email: String,
    pub id: String,
    pub discord_id: Option<i64>,
}

impl PatreonCacheRow {
    pub async fn get(pool: &PgPool, key: &str) -> Result<Option<Self>> {
        let row = if let Ok(id) = key.parse::<u64>() {
            Self::get_from_id(pool, id).await.unwrap()
        } else {
            Self::get_from_email(pool, key).await.unwrap()
        };

        Ok(row)
    }

    pub async fn get_from_email(pool: &PgPool, email: &str) -> Result<Option<Self>> {
        let row = sqlx::query_as!(
            PatreonCacheRow,
            "SELECT * FROM patreon_cache WHERE email = $1",
            email,
        )
        .fetch_optional(pool)
        .await
        .unwrap();

        Ok(row)
    }

    pub async fn get_from_id(pool: &PgPool, id: u64) -> Result<Option<Self>> {
        let row = sqlx::query_as!(
            PatreonCacheRow,
            "SELECT * FROM patreon_cache WHERE id = $1",
            id as i64,
        )
        .fetch_optional(pool)
        .await
        .unwrap();

        Ok(row)
    }
}

pub async fn patreon_member(pool: &PgPool, key: &str, force: bool) -> Result<MemberResponse> {
    let row = PatreonCacheRow::get(pool, key).await.unwrap().unwrap();

    let api_key = env::var("PATREON_TOKEN").unwrap();

    let client = PatreonClientBuilder::new(api_key, PATREON_CLIENT_ID)
        .build()
        .unwrap();

    let member = client
        .member(&row.id, MemberInclude::CURRENTLY_ENTITLED_TIERS)
        .await
        .unwrap();

    Ok(member)
}
