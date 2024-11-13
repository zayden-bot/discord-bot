use std::fmt;

use reqwest::Client;
use serde::Deserialize;

use crate::{Error, Result, SERVER_URL};

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    pub lifetime_support_cents: i32,
    pub tiers: Vec<Tier>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Tier {
    pub attributes: TierAttributes,
}

#[derive(Deserialize, Debug, Default)]

pub struct TierAttributes {
    pub amount_cents: i32,
}

#[derive(Debug)]
pub struct PatreonUser {
    pub lifetime_support: i32,
    pub tier: i32,
}

impl PatreonUser {
    pub async fn get(client: &Client, key: impl fmt::Display, force: bool) -> Result<Self> {
        let res = client
            .get(format!("{SERVER_URL}/api/v1/patreon/user/{key}"))
            .query(&[("force", force)])
            .send()
            .await?;

        println!("{:?}", res);

        if res.status().is_success() {
            let user: UserResponse = res.json().await?;
            let highest_tier = user
                .tiers
                .into_iter()
                .map(|tier| tier.attributes.amount_cents)
                .max()
                .unwrap_or_default();

            Ok(PatreonUser {
                lifetime_support: user.lifetime_support_cents / 100,
                tier: highest_tier / 100,
            })
        } else {
            Err(Error::PatreonAccountNotFound(key.to_string()))
        }
    }
}
