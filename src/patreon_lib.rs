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
    pub email: Option<String>,
    pub lifetime_support_cents: Option<i32>,
}

pub async fn get_user(email: &str, force: bool) -> Result<MemberAttributes> {
    let res = Client::new()
        .post(&format!("{}/api/v1/patreon/get_user", SERVER_URL))
        .json(&UserRequest {
            email: email.to_string(),
            force,
        })
        .send()
        .await?;

    if res.status().is_success() {
        let attributes = res.json().await?;
        Ok(attributes)
    } else {
        Err(Error::InvalidEmail)
    }
}
