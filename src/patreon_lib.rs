use std::fmt;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{Error, Result, SERVER_URL};

#[derive(Debug, Serialize)]
struct UserRequest {
    email: String,
    force: bool,
}

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    pub lifetime_support_cents: i32,
    pub tier: Tier,
}

#[derive(Deserialize, Debug)]
pub struct Tier {
    pub amount_cents: i32,
}

pub async fn get_user(
    client: &Client,
    key: impl fmt::Display,
    force: bool,
) -> Result<UserResponse> {
    let res = client
        .get(&format!("{SERVER_URL}/api/v1/patreon/user/{key}"))
        .query(&[("force", force)])
        .send()
        .await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        Err(Error::PatreonAccountNotFound(key.to_string()))
    }
}
