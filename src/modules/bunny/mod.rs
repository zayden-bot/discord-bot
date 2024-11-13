use bunny_cdn_wrapper::BunnyStorage;
use std::env;

use crate::Result;

pub async fn latest_download_link(app_name: &str, platform: &str) -> Result<String> {
    let bunny_storage = BunnyStorage::new(
        "collegekingsstorage",
        env::var("BUNNY_READ_ONLY_KEY")?,
        "de",
    )?;
    let files = bunny_storage
        .list(&format!(
            "__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/{}/",
            app_name
        ))
        .await?;

    let latest_file = files
        .into_iter()
        .filter(|file| file.object_name.ends_with(&format!("{}.zip", platform)))
        .map(|file| file.object_name)
        .max_by(|a, b| {
            a.split('-')
                .nth(1)
                .unwrap()
                .split('.')
                .map(|part| part.parse::<u32>().unwrap())
                .zip(
                    b.split('-')
                        .nth(1)
                        .unwrap()
                        .split('.')
                        .map(|part| part.parse::<u32>().unwrap_or(0)),
                )
                .map(|(a_part, b_part)| a_part.cmp(&b_part))
                .find(|cmp| *cmp != std::cmp::Ordering::Equal)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

    Ok(format!("https://collegekings.b-cdn.net/__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/{app_name}/{}", latest_file.unwrap()))
}
