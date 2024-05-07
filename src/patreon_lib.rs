use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{Error, Result, SERVER_URL};

#[derive(Debug, Serialize)]
struct UserRequest {
    email: String,
    force: bool,
}

#[derive(Deserialize, Debug)]
pub struct MemberAttributes {
    pub currently_entitled_amount_cents: Option<i32>,
    pub lifetime_support_cents: Option<i32>,
}

pub async fn get_user(
    client: &Client,
    key: impl std::fmt::Display,
    force: bool,
) -> Result<MemberAttributes> {
    let res = client
        .get(&format!("{SERVER_URL}/api/v1/patreon/user/{key}"))
        .query(&[("force", force)])
        .send()
        .await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        Err(Error::InvalidEmail)
    }
}
