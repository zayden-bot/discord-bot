use std::{thread::sleep, time::Duration};

use serde::Deserialize;
use sqlx::PgPool;
use url::Url;

const URL: &str = "https://mee6.xyz/api/plugins/levels/leaderboard/745662812335898806";

#[derive(Debug, Deserialize)]
struct Mee6Response {
    players: Vec<Mee6Player>,
}

#[derive(Debug, Deserialize)]
struct Mee6Player {
    id: String,
    message_count: u64,
    xp: u64,
}

pub async fn sync(pool: &PgPool) {
    let mut url = Url::parse(URL).unwrap();

    let mut page = 0;

    let mut txn = pool.begin().await.unwrap();

    loop {
        println!("Fetching page {}", page);

        if page != 0 {
            url.set_query(Some(&format!("page={}", page)));
        }

        let res = reqwest::get(url.clone()).await.unwrap();

        let body = res.json::<Mee6Response>().await.unwrap();

        if body.players.is_empty() {
            break;
        }

        for player in body.players {
            sqlx::query!(
                "INSERT INTO levels (id, message_count, total_xp) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET message_count = $2, total_xp = $3",
                player.id.parse::<i64>().unwrap(),
                player.message_count as i64,
                player.xp as i64
            )
            .execute(&mut *txn)
            .await
            .unwrap();
        }

        page += 1;

        sleep(Duration::from_secs(2));
    }

    txn.commit().await.unwrap();
}
