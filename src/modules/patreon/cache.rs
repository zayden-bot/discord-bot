use std::env;
use std::str::FromStr;

use async_trait::async_trait;
use cron::Schedule;
use patreon_api::pagination_options::PaginationOptions;
use patreon_api::patreon_client::{PatreonClient, PatreonClientBuilder};
use patreon_api::types::includes::MemberInclude;
use patreon_api::types::response::{Cursors, MemberIncluded, Meta, Pagination};
use serenity::all::Context;

use crate::cron::CronJob;
use crate::sqlx_lib::PostgresPool;
use crate::Result;

pub const PATREON_CLIENT_ID: &str =
    "co3TJ3lwqHN5WSVuIBiDNhfQv28V4FR-z6g-_fIogDzj_Um09DoWLGE5rvAJeTQd";
pub const CAMPAIGN_ID: u64 = 5167485;

pub struct PatreonCache;

#[async_trait]
impl CronJob for PatreonCache {
    fn schedule(&self) -> Schedule {
        Schedule::from_str("0 0 0 * * *").unwrap()
    }

    async fn action(&self, ctx: &Context) -> Result<()> {
        let client =
            PatreonClientBuilder::new(env::var("PATREON_TOKEN").unwrap(), PATREON_CLIENT_ID)
                .build()
                .unwrap();

        let pool = PostgresPool::get(ctx).await;

        let transaction = pool.begin().await.unwrap();

        update_cache(
            &client,
            transaction,
            Some(PaginationOptions::new().count(1000)),
        )
        .await;

        Ok(())
    }
}

async fn update_cache(
    client: &PatreonClient,
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>,
    mut pagination: Option<PaginationOptions>,
) {
    let response = client
        .campaign_members(CAMPAIGN_ID, MemberInclude::USER, pagination)
        .await
        .unwrap();

    let data_iter = response.data.iter();

    let included_iter = response.included.iter().filter_map(|i| match i {
        MemberIncluded::User(user) => Some(user),
        _ => None,
    });

    for (data, included) in data_iter
        .zip(included_iter)
        .filter(|(d, _)| d.attributes.email.is_some())
    {
        let email = data.attributes.email.as_ref().unwrap();
        let id = &data.id;
        let discord_id = included
            .attributes
            .social_connections
            .get("discord")
            .and_then(|sc| {
                sc.as_ref()
                    .map(|sc| sc.user_id.as_ref().map(|id| id.parse::<i64>().unwrap()))
            })
            .flatten();

        sqlx::query!(
            "INSERT INTO patreon_cache (email, id, discord_id)
             VALUES ($1, $2, $3)
             ON CONFLICT (email) DO UPDATE
             SET id = EXCLUDED.id, discord_id = EXCLUDED.discord_id",
            email,
            id,
            discord_id
        )
        .execute(&mut *transaction)
        .await
        .unwrap();
    }

    if let Some(Meta {
        pagination:
            Pagination {
                cursors: Cursors { next: Some(next) },
                ..
            },
    }) = response.meta
    {
        std::thread::sleep(std::time::Duration::from_secs(3));
        pagination = Some(PaginationOptions::new().count(1000).cursor(next));
        Box::pin(update_cache(client, transaction, pagination)).await;
    }
}
